use serde::{Deserialize, Serialize};

/// Config struct for the block client.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// URL of the block service.
    pub url: String,
    /// Authorization token.
    pub token: String,
    /// Stream name.
    pub stream_name: String,
    /// Connection window size.
    pub connection_window_size: u32,
    /// Stream window size.
    pub stream_window_size: u32,
    /// Request timeout.
    pub request_timeout: u64,
    /// Connect timeout.
    pub connect_timeout: u64,
    /// Buffer size.
    pub buffer_size: usize,
    /// Max message size.
    pub max_message_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: "http://localhost:4300".to_string(),
            token: String::new(),
            stream_name: "v2_mainnet_near_blocks".to_string(),
            connection_window_size: 0,
            stream_window_size: 0,
            request_timeout: 0,
            connect_timeout: 0,
            buffer_size: 0,
            max_message_size: 0,
        }
    }
}
