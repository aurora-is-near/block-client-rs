# Block Client Library

The `block-client-rs` is the implementation of the client for the block storage service written in Rust. The purpose of
the library is the following:

- Fetch blocks from the GRPC stream.
- Push blocks to the GRPC stream.

## Usage

```rust
use block_client::{BlockClient, BlocksRequestBuilder, StartPolicy, Config};

fn main() {
    let config = Config {
        url: "http://block_storage.aurora.dev:4300".to_string(),
        token: "auth_token".to_string(),
        connection_window_size: 64 * 1024 * 1024,
        stream_window_size: 64 * 1024 * 1024,
        request_timeout: 10,
        connect_timeout: 10,
        buffer_size: 64 * 1024 * 1024,
        max_message_size: 1024 * 1024 * 1024,
    };

    let request = BlocksRequestBuilder::new()
        .with_stream_name("v2_mainnet_near_blocks")
        .with_start_policy(StartPolicy::StartOnLatestAvailable)
        .build();
    let mut blocks = BlockClient::new(config).unwrap().get_block_stream(request).await.unwrap();
    let block_message = blocks.next().await.unwrap();
}
```
