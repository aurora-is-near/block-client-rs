use crate::types::proto::block_message::Payload;
use crate::types::proto::receive_blocks_response::Response;
use crate::types::proto::ReceiveBlocksResponse;
use crate::types::BlockMessage;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::{async_trait, Streaming};

#[async_trait]
pub trait ReadStream {
    type Item: Send;

    async fn next(&mut self) -> anyhow::Result<Self::Item>;
}

pub struct BlocksStream {
    stream: Streaming<ReceiveBlocksResponse>,
}

impl BlocksStream {
    #[must_use]
    pub const fn new(stream: Streaming<ReceiveBlocksResponse>) -> Self {
        Self { stream }
    }
}

#[async_trait]
impl ReadStream for BlocksStream {
    type Item = BlockMessage;

    async fn next(&mut self) -> anyhow::Result<Self::Item> {
        self.stream
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Stream is empty"))?
            .map_err(Into::into)
            .and_then(TryFrom::try_from)
    }
}

impl TryFrom<ReceiveBlocksResponse> for BlockMessage {
    type Error = anyhow::Error;

    fn try_from(value: ReceiveBlocksResponse) -> anyhow::Result<Self> {
        match value.response {
            None => anyhow::bail!("Received empty response"),
            Some(response) => match response {
                Response::Message(msg) => msg.message.map(|m| {
                    Self {
                        height: m.id.map_or(0, |id| id.height),
                        payload: m.payload.map(|p| {
                            let Payload::RawPayload(p) = p;
                            p
                        }).unwrap_or_default(),
                        format: m.format.into(),
                    }
                }).ok_or_else(|| anyhow::anyhow!("Received message without payload")),
                Response::Done(_) => anyhow::bail!("The stream is finished"),
                Response::Error(e) => anyhow::bail!("Received error: {e:?}"),
                Response::ZstdDict(_) => anyhow::bail!("The wrong response type received"),
            },
        }
    }
}
