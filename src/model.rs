use actix::{Addr, Message};
use serde_derive::{Deserialize, Serialize};

/// PushEvent represents a github PushEvent. The reference
/// to the PushEvent is contained within the struct. This will be used to monitor
/// a github webhook.
#[derive(Debug, Clone, Deserialize, Serialize, Message)]
pub struct PushEvent {
    pub reference: String,
}
