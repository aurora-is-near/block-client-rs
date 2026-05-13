pub mod bus_message;
pub mod proto;
pub mod request;

/// Message with block payload.
#[derive(Debug)]
pub struct BlockMessage {
    /// Height of the block.
    pub height: u64,
    /// Payload of the block.
    pub payload: Vec<u8>,
    /// Block payload format.
    pub format: BlockPayloadFormat,
}

/// Block payload format.
#[derive(Debug)]
pub enum BlockPayloadFormat {
    /// JSON -> LZ4 -> CBOR bytes -> borealis envelope (whole blocks)
    NearBlockV2,
    /// CBOR STRUCT -> borealis envelope (whole blocks)
    AuroraBlockV2,
    /// protobuf (block headers + block shards)
    NearBlockV3,
    /// Unknown format
    Unknown,
}

impl From<i32> for BlockPayloadFormat {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::NearBlockV2,
            1 => Self::AuroraBlockV2,
            2 => Self::NearBlockV3,
            _ => Self::Unknown,
        }
    }
}
