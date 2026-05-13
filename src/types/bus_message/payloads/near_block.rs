use crate::types::bus_message::bus_serde::{DecodeError, EncodeError, FromCbor, ToCbor};
use crate::types::bus_message::message_type::MessageType;
use crate::types::bus_message::payloads::Payload;

pub use aurora_refiner_types::near_block::NEARBlock;

impl ToCbor for NEARBlock {
    fn to_cbor(self) -> Result<serde_cbor::Value, EncodeError> {
        let json_bytes = serde_json::to_vec(&self)?;
        let json_comp = crate::types::bus_message::compression::compress(&json_bytes)?;

        Ok(serde_cbor::Value::Bytes(json_comp))
    }
}

impl FromCbor for NEARBlock {
    fn from_cbor(input: serde_cbor::Value) -> Result<Self, DecodeError> {
        let json_comp = Vec::<u8>::from_cbor(input)?;
        let json_bytes = crate::types::bus_message::compression::decompress(&json_comp)?;
        let result = serde_json::from_slice(&json_bytes)?;

        Ok(result)
    }
}

impl Payload for NEARBlock {
    const MESSAGE_TYPE: MessageType = MessageType::NEARBlockEvent;

    fn unique_id(&self) -> Option<[u8; 16]> {
        let height_bytes = self.block.header.height.to_be_bytes();
        let mut result = [0u8; 16];
        result[0..8].copy_from_slice(&height_bytes);
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::{FromCbor, NEARBlock, ToCbor};

    #[test]
    fn test_near_serde_round_trip() {
        let near_block: NEARBlock = {
            let file = std::fs::File::open("src/res/near_block_66381607.json").unwrap();
            serde_json::from_reader(file).unwrap()
        };
        let expected_bytes = serde_json::to_vec(&near_block).unwrap();
        let value = near_block.to_cbor().unwrap();
        let deser = NEARBlock::from_cbor(value).unwrap();
        assert_eq!(expected_bytes, serde_json::to_vec(&deser).unwrap());
    }
}
