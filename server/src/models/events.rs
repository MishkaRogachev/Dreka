use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
#[derive(Clone)]
pub enum ClentEvent {
    // Communication service events
    SetLinkConnected { link_id: String, connected: bool },
    ForgetConnection { link_id: String },
}
