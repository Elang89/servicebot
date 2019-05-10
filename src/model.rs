/// This is the models file. This file defines all the structures that will be used throughout this applicaion.
use serde_derive::{Deserialize, Serialize};

/// AppState is the global registry for each different actor, this struct
/// contains the addresses of the different actor classes in this application.
pub struct AppState {}

/// PushEvent represents a github PushEvent. In this case on the reference
/// to the PushEvent is contained within the struct. This will be used to monitor
/// a github webhook.
#[derive(Debug, Deserialize, Serialize)]
pub struct PushEvent {
    pub reference: String,
}
