use std::str::FromStr;
use std::time::Duration;
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::{Extensions, IntoRequest, Request, Response};

use crate::stream::read::BlocksStream;
use crate::types::proto::blocks_provider_client::BlocksProviderClient;
use crate::types::request::BlocksRequest;

pub use config::Config;

mod config;
pub mod stream;
pub mod types;

const CONCURRENCY_LIMIT: usize = 256;

/// Client for fetching blocks from the blocks service.
pub struct BlockClient {
    client: BlocksProviderClient<tonic::transport::Channel>,
    token: String,
}

impl BlockClient {
    /// Creates a new instance of the `BlockClient`.
    ///
    /// # Errors
    ///
    /// Returns an error if the channel cannot be created.
    pub fn new(config: Config) -> anyhow::Result<Self> {
        let channel = Self::create_channel(&config)?;
        let client = BlocksProviderClient::new(channel)
            .max_decoding_message_size(config.max_message_size)
            .max_encoding_message_size(config.max_message_size);


        Ok(Self {
            client,
            token: config.token,
        })
    }

    /// Fetches a stream of the blocks.
    ///
    /// # Errors
    ///
    /// Returns an error if the stream cannot be received.
    pub async fn get_block_stream(
        &mut self,
        request: BlocksRequest,
    ) -> anyhow::Result<BlocksStream> {
        let request = self.create_request(request);
        let stream = self
            .client
            .receive_blocks(request.into_request())
            .await
            .map(Response::into_inner)?;

        Ok(BlocksStream::new(stream))
    }

    fn create_request<T, R: From<T>>(&self, request: T) -> impl IntoRequest<R> {
        Request::from_parts(Self::metadata(&self.token), Extensions::new(), request.into())
    }

    fn create_channel(config: &Config) -> anyhow::Result<tonic::transport::Channel> {
        tonic::transport::Channel::from_shared(config.url.clone())
            .map(|channel| {
                channel
                    .timeout(Duration::from_secs(config.request_timeout))
                    .connect_timeout(Duration::from_secs(config.connect_timeout))
                    .keep_alive_while_idle(true)
                    .initial_connection_window_size(config.connection_window_size)
                    .initial_stream_window_size(config.stream_window_size)
                    .buffer_size(config.buffer_size)
                    .concurrency_limit(CONCURRENCY_LIMIT)
                    .connect_lazy()
            })
            .map_err(Into::into)
    }

    fn metadata(token: &str) -> MetadataMap {
        let mut metadata = MetadataMap::new();

        metadata.insert(
            "authorization",
            MetadataValue::from_str(&format!("Bearer {token}")).unwrap(),
        );

        metadata
    }
}
