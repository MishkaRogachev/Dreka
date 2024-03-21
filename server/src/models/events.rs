use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum ClentEvent {
    // Communication service events
    SetLinkEnabled { link_id: String, connected: bool },
}
