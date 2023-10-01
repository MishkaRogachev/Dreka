use std::sync::Arc;
use tokio::time;

use crate::db::persistence;
use crate::models::links;
use crate::protocols::mavlink;

const REFRERSH_CONNECTIONS_INTERVAL: time::Duration = time::Duration::from_secs(1);

pub async fn start(persistence: Arc<persistence::Persistence>) {
    let mut interval = time::interval(REFRERSH_CONNECTIONS_INTERVAL);
    interval.tick().await; // skip first tick

    loop {
        tokio::time::sleep(REFRERSH_CONNECTIONS_INTERVAL).await;
        refresh_connections(&persistence).await;
    }
}

async fn refresh_connections(persistence: &Arc<persistence::Persistence>) {
    let response = persistence.read_all::<links::LinkDescription>("links").await;
    match response {
        Ok(links) => {
            for link in links {
                let result = start_link(&link).await;
                if let Err(err) = result {
                    print!("Link error: {}", err.to_string())
                }
            }
        },
        Err(err) => panic!("Persistence error: {}", err.to_string()),
    }
}

pub async fn start_link(link: &links::LinkDescription) -> std::io::Result<()> {
    match &link.protocol {
        links::LinkProtocol::Mavlink { link_type, protocol_version } => {
            return mavlink::protocol::connect(link_type, protocol_version).await;
        },
        // NOTE: other protocols should be handled here
    }
}
