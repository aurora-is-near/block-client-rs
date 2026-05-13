#![allow(clippy::redundant_pub_crate)]

use aurora_refiner_types::near_block::NEARBlock;
use block_client_rs::stream::read::ReadStream;
use block_client_rs::types::bus_message::BusMessage;
use block_client_rs::types::request::{BlocksRequestBuilder, CatchupPolicy, DeliverySettings, StartPolicy};
use block_client_rs::{BlockClient, Config};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// GRPC server URL
    #[arg(long)]
    url: String,
    /// Authorization token
    #[arg(long)]
    token: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = Config {
        url: args.url,
        token: args.token,
        stream_name: "v2_mainnet_near_blocks".to_string(),
        connection_window_size: 64 * 1024 * 1024,
        stream_window_size: 64 * 1024 * 1024,
        request_timeout: 10,
        connect_timeout: 10,
        buffer_size: 64 * 1024 * 1024,
        max_message_size: 1024 * 1024 * 1024,
    };

    let ctrl = tokio::signal::ctrl_c();

    let request = BlocksRequestBuilder::new()
        .with_stream_name(&config.stream_name)
        .with_start_policy(StartPolicy::StartOnLatestAvailable)
        .with_catchup_policy(CatchupPolicy::CatchupWait)
        .with_delivery_settings(DeliverySettings {
            exclude_payload: false,
            allow_compression: 1,
        })
        .build();
    let mut blocks = BlockClient::new(config).unwrap().get_block_stream(request).await.unwrap();

    let total_start = std::time::Instant::now();
    let mut total_received = 0;

    tokio::pin!(ctrl);

    loop {
        tokio::select! {
            response = blocks.next() => {
                match response {
                    Ok(msg) => {
                        total_received += 1;
                        let time: chrono::DateTime<chrono::Utc> = std::time::SystemTime::now().into();
                        let block = BusMessage::<NEARBlock>::deserialize(msg.payload.as_slice()).unwrap();
                        assert_eq!(block.payload.block.header.height, msg.height);
                        println!("[{time}] received block with height: {}", msg.height);
                    }
                    Err(e) => println!("Error: {e:?}"),
                }
            },
            _ = &mut ctrl => {
                println!("Ctrl+C received, stopping...\n{total_received} blocks received during: {} seconds", total_start.elapsed().as_secs());
                break;
            }
        }
    }
}
