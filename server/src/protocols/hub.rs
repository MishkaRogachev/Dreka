use std::sync::Arc;
use std::collections::HashMap;
use tokio::time;

use crate::db::persistence;
use crate::models::links;
use crate::protocols::mavlink;
use super::common;

type LickConnection = Box<dyn common::Connection + Send + Sync>;
type LinkConnections = HashMap<String, LickConnection>;

const REFRERSH_CONNECTIONS_INTERVAL: time::Duration = time::Duration::from_secs(1);

pub async fn start(persistence: Arc<persistence::Persistence>) {
    let mut interval = time::interval(REFRERSH_CONNECTIONS_INTERVAL);
    interval.tick().await;  // skip first tick

    let mut link_connections: LinkConnections = HashMap::new();
    loop {
        tokio::time::sleep(REFRERSH_CONNECTIONS_INTERVAL).await;
        refresh_connections(&persistence, &mut link_connections).await;
    }
}

// TODO: mutable thread-safe connections map

async fn refresh_connections(persistence: &Arc<persistence::Persistence>, link_connections: &mut LinkConnections) {
    let response = persistence.read_all::<links::LinkDescription>("links").await;
    match response {
        Ok(links) => {
            let mut link_ids: Vec<String> = Vec::new();
            for link in links {
                let link_id = &link.id.clone().unwrap().to_string();
                link_ids.push(link_id.to_owned());

                // Add connections for (new) links
                if !link_connections.contains_key(link_id) {
                    link_connections.insert(link_id.to_owned(), create_connection(&link));
                }

                // Update connection status for link connections
                let connection = link_connections.get_mut(link_id).unwrap();
                if link.enabled && !connection.is_connected() {
                    if let Err(err) = connection.connect().await {
                        println!("Connect error: {}", err.to_string());
                    }
                } else if !link.enabled && connection.is_connected() {
                    if let Err(err) = connection.disconnect().await {
                        println!("Disconnect error: {}", err.to_string());
                    }
                }
            }

            // Disconnect removed links
            for (link_id, connection) in link_connections.iter_mut() {
                if !link_ids.contains(&link_id) {
                    if let Err(err) = connection.disconnect().await {
                        println!("Disconnect (on remove) error: {}", err.to_string());
                    }
                }
            }

            // Remove connections for deleted links
            link_connections.retain(|link_id, _| link_ids.contains(&link_id));
        },
        Err(err) => panic!("Persistence error: {}", err.to_string()),
    }
}

fn create_connection(link: &links::LinkDescription) -> LickConnection {
    match &link.protocol {
        links::LinkProtocol::Mavlink { link_type, protocol_version } => {
            Box::new(mavlink::connection::MavlinkConnection::new(link_type, protocol_version))
        },
        // NOTE: other protocols should be handled here
    }
}
