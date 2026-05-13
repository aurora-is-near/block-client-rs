use crate::types::proto::block_message::Id;
use crate::types::proto::{BlockMessageDeliverySettings, BlockStreamDeliverySettings, ReceiveBlocksRequest};

/// Builder for `BlocksRequest`.
#[derive(Debug, Default)]
pub struct BlocksRequestBuilder {
    stream_name: String,
    start_policy: StartPolicy,
    catchup_policy: CatchupPolicy,
    stop_policy: StopPolicy,
    delivery_settings: Option<DeliverySettings>,
}

impl BlocksRequestBuilder {
    /// Creates a new `BlocksRequestBuilder` with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the stream name.
    #[must_use]
    pub fn with_stream_name<S: AsRef<str>>(mut self, stream_name: S) -> Self {
        self.stream_name = stream_name.as_ref().to_string();
        self
    }

    /// Sets the start policy.
    #[must_use]
    pub const fn with_start_policy(mut self, start_policy: StartPolicy) -> Self {
        self.start_policy = start_policy;
        self
    }

    /// Sets the catchup policy.
    #[must_use]
    pub const fn with_catchup_policy(mut self, catchup_policy: CatchupPolicy) -> Self {
        self.catchup_policy = catchup_policy;
        self
    }

    /// Sets the stop policy.
    #[must_use]
    pub const fn with_stop_policy(mut self, stop_policy: StopPolicy) -> Self {
        self.stop_policy = stop_policy;
        self
    }

    /// Delivery settings.
    #[must_use]
    pub const fn with_delivery_settings(mut self, delivery_settings: DeliverySettings) -> Self {
        self.delivery_settings = Some(delivery_settings);
        self
    }

    /// Builds the `BlocksRequest`.
    #[must_use]
    pub fn build(self) -> BlocksRequest {
        BlocksRequest {
            stream_name: self.stream_name,
            start_policy: self.start_policy,
            catchup_policy: self.catchup_policy,
            stop_policy: self.stop_policy,
            delivery_settings: self.delivery_settings,
        }
    }
}


#[derive(Debug, Default)]
pub struct BlocksRequest {
    /// Name of the stream to receive blocks from
    pub stream_name: String,
    /// Start policy
    pub start_policy: StartPolicy,
    /// Catchup policy
    pub catchup_policy: CatchupPolicy,
    /// Stop policy
    pub stop_policy: StopPolicy,
    /// Delivery settings
    pub delivery_settings: Option<DeliverySettings>,
}

#[derive(Debug, Default)]
pub enum StartPolicy {
    /// Start on the earliest available message
    #[default]
    StartOnEarliestAvailable,
    /// Start on the latest available message
    StartOnLatestAvailable,
    /// Start exactly on target, return error if no such target
    StartExactlyOnTarget(u64),
    /// Start on a message which comes exactly after the target, return error if no such target
    StartExactlyAfterTarget(u64),
    /// Start on the earliest available message that is greater or equal to the target
    StartOnClosestToTarget(u64),
    /// Start on the earliest available message that is strictly greater than target
    StartOnEarliestAfterTarget(u64),
}

impl StartPolicy {
    #[must_use]
    pub const fn policy(&self) -> i32 {
        match self {
            Self::StartOnEarliestAvailable => 0,
            Self::StartOnLatestAvailable => 1,
            Self::StartExactlyOnTarget(_) => 2,
            Self::StartExactlyAfterTarget(_) => 3,
            Self::StartOnClosestToTarget(_) => 4,
            Self::StartOnEarliestAfterTarget(_) => 5,
        }
    }

    #[must_use]
    pub const fn target(&self) -> Option<u64> {
        match self {
            Self::StartOnEarliestAvailable | Self::StartOnLatestAvailable => None,
            Self::StartExactlyOnTarget(target)
            | Self::StartExactlyAfterTarget(target)
            | Self::StartOnClosestToTarget(target)
            | Self::StartOnEarliestAfterTarget(target) => Some(*target),
        }
    }
}

#[derive(Debug, Default)]
pub enum StopPolicy {
    /// Never stop, consume new blocks as they arrive
    #[default]
    Never,
    /// Don't send messages greater than target
    AfterTarget(u64),
    /// Don't send messages greater or equal to the target
    BeforeTarget(u64),
}
impl StopPolicy {
    const fn policy(&self) -> i32 {
        match self {
            Self::Never => 0,
            Self::AfterTarget(_) => 1,
            Self::BeforeTarget(_) => 2,
        }
    }

    const fn target(&self) -> Option<u64> {
        match self {
            Self::Never => None,
            Self::AfterTarget(target) | Self::BeforeTarget(target) => Some(*target),
        }
    }
}

/// Defines how service should behave if the start target is not yet available.
#[derive(Debug, Default)]
pub enum CatchupPolicy {
    /// Return an error if the catchup is needed
    #[default]
    CatchupPanic,
    /// Don't send anything until catch up
    CatchupWait,
    /// Stream normally from whatever is available before the start target
    CatchupStream,
}

impl CatchupPolicy {
    const fn policy(&self) -> i32 {
        match self {
            Self::CatchupPanic => 0,
            Self::CatchupWait => 1,
            Self::CatchupStream => 2,
        }
    }
}

#[derive(Debug, Default)]
pub struct DeliverySettings {
    pub exclude_payload: bool,
    pub allow_compression: i32,
}

impl From<DeliverySettings> for BlockMessageDeliverySettings {
    fn from(value: DeliverySettings) -> Self {
        Self {
            exclude_payload: value.exclude_payload,
            allow_compression: value.allow_compression,
            require_format: None,
        }
    }
}

impl From<BlocksRequest> for ReceiveBlocksRequest {
    fn from(value: BlocksRequest) -> Self {
        Self {
            stream_name: value.stream_name,
            stream_origin: String::new(),
            start_policy: value.start_policy.policy(),
            start_target: value.start_policy.target().map(|height| Id {
                kind: 0, // 0 - whole block
                height,
                shard_id: 0, // doesn't work now
            }),
            stop_policy: value.stop_policy.policy(),
            stop_target: value.stop_policy.target().map(|height| Id {
                kind: 0, // 0 - whole block
                height,
                shard_id: 0, // doesn't work now
            }),
            delivery_settings: value.delivery_settings.map(|setting| {
                BlockStreamDeliverySettings {
                    filter: None,
                    content: Some(setting.into()),
                }
            }),
            catchup_policy: value.catchup_policy.policy(),
            catchup_delivery_settings: None,
            cached_zstd_dicts_sha3_hashes: vec![],
        }
    }
}
