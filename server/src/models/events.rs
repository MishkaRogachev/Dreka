use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum ClentEvent {
    // Communication service events
    SetLinkConnected { link_id: String, connected: bool },
}
