use std::{sync::Arc, collections::HashMap, time::Duration};
use tokio::{sync::broadcast, time};

use crate::{datasource::db, models::{self, communication::{self, LinkDescription}, events::ClentEvent}};
use super::{default_links::create_dafault_links, traits, mavlink::connection::MavlinkConnection};

type LickConnection = Box<dyn traits::IConnection + Send + Sync>;
type LinkConnections = HashMap<String, LickConnection>;

const CHECK_CONNECTIONS_INTERVAL: Duration = Duration::from_millis(100);

pub struct Service {
    repository: Arc<db::Repository>,
    rx: broadcast::Receiver<models::events::ClentEvent>,
    link_connections: LinkConnections
}

#[derive(Debug)]
pub enum ServiceError {
    Db(db::DbError),
    Connection(traits::ConnectionError),
}

impl Service {
    pub fn new(repository: Arc<db::Repository>, rx: broadcast::Receiver<models::events::ClentEvent>) -> Self {
        Self { repository, rx, link_connections: LinkConnections::new() }
    }

    pub async fn start(&mut self) -> Result<(), ServiceError> {
        // Load all links
        let links = self.load_links().await?;

        for link in links {
            let link_id = link.id.clone().expect("Link must have an id");
            // Default status on start
            self.default_status(&link_id).await?;

            // Autoconect specified one
            if link.autoconnect {
                self.connect_link(&link_id).await?;
            }
        }
    
        let mut interval = time::interval(CHECK_CONNECTIONS_INTERVAL);
        interval.tick().await;  // skip first tick

        loop { // NOTE: NO await? in this loop, use logger!
            interval.tick().await;

            // Listen requests from client
            match self.rx.try_recv() {
                Ok(event) => {
                    let result = self.handle_event(event).await;
                    if let Err(err) = result {
                        println!("handle event error: {}", err);
                    }
                },
                Err(err) => {
                    if err != tokio::sync::broadcast::error::TryRecvError::Empty {
                        println!("RX error: {}", err);
                    }
                }
            }

            // Collect link statistics
            for (link_id, connection) in self.link_connections.iter_mut() {
                let status = communication::LinkStatus {
                    id: link_id.clone(),
                    is_connected: connection.is_connected(),
                    is_online: connection.is_online(),
                };

                let result = self.repository.create_or_update("link_statuses", &status).await;
                if let Err(err) = result {
                    return Err(ServiceError::Db(err));
                }
                println!("-----> {:?}", status);
            }
        }
    }

    async fn load_links(&self) -> Result<Vec<LinkDescription>, ServiceError> {
        let mut links: Vec<LinkDescription>;

        match self.repository.read_all::<communication::LinkDescription>("link_descriptions").await {
            Ok(read_links) => links = read_links,
            Err(err) => return Err(ServiceError::Db(err)),
        }

        if links.len() < 1 {
            match create_dafault_links(&self.repository).await {
                Ok(default_links) => links = default_links,
                Err(err) => return Err(ServiceError::Db(err)),
            }
        }
        Ok(links)
    }
    
    async fn connect_link(&mut self, link_id: &str) -> Result<(), ServiceError> {
        if !self.link_connections.contains_key(link_id) {
            let link = self.repository.read("link_descriptions", link_id).await;
            if let Err(err) = link {
                return Err(ServiceError::Db(err));
            }
            self.link_connections.insert(link_id.to_string(), create_connection(&link.unwrap()));
        }

        let connection = self.link_connections.get_mut(link_id).unwrap();
        if let Err(err) = connection.connect().await {
            return Err(ServiceError::Connection(err));
        }

        let status = communication::LinkStatus {
            id: link_id.into(),
            is_connected: connection.is_connected(),
            is_online: connection.is_online(),
        };

        let result = self.repository.create_or_update("link_statuses", &status).await;
        if let Err(err) = result {
            return Err(ServiceError::Db(err));
        }
        Ok(())
    }

    async fn disconnect_link(&mut self, link_id: &str) -> Result<(), ServiceError> {
        if !self.link_connections.contains_key(link_id) {
            println!("No connection found for link {}", link_id);
            return Ok(());
        }
        
        let connection = self.link_connections.get_mut(link_id).unwrap();
        if let Err(err) = connection.disconnect().await {
            return Err(ServiceError::Connection(err))
        }

        return self.default_status(link_id).await;
    }

    async fn handle_event(&mut self, event: models::events::ClentEvent) -> Result<(), ServiceError> {
        match event {
            ClentEvent::SetLinkConnected { link_id, connected } => {
                if connected {
                    return self.connect_link(&link_id).await;
                } else {
                    return self.disconnect_link(&link_id).await;
                }
            }
            ClentEvent::ForgetConnection { link_id } => {
                self.link_connections.remove(&link_id);
                return Ok(());
            }
        }
    }

    async fn default_status(&self, link_id: &str) -> Result<(), ServiceError> {
        let status = communication::LinkStatus {
            id: link_id.into(),
            is_connected: false,
            is_online: false,
        };

        let result = self.repository.create_or_update("link_statuses", &status).await;
        if let Err(err) = result {
            return Err(ServiceError::Db(err));
        }
        Ok(())
    }
}

fn create_connection(link: &communication::LinkDescription) -> LickConnection {
    match &link.protocol {
        communication::LinkProtocol::Mavlink { link_type, protocol_version } => {
            Box::new(MavlinkConnection::new(link_type, protocol_version))
        },
        // NOTE: other protocols should be handled here
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServiceError::Db(err) => write!(f, "{}", err),
            ServiceError::Connection(err) => write!(f, "{}", err),
        }
    }
}



// use std::collections::HashMap;
// use tokio::time;

// use crate::models::communication;
// use super::traits;

// type LickConnection = Box<dyn traits::IConnection + Send + Sync>;
// type LinkConnections = HashMap<String, LickConnection>;


    //services::links::check_and_create_links(&db).await?;
    //let hub = tokio::spawn(services::hub::start(db.clone()));

// const REFRERSH_CONNECTIONS_INTERVAL: time::Duration = time::Duration::from_secs(1);

// pub async fn start(repo: Arc<db::Repository>) {
//     let mut interval = time::interval(REFRERSH_CONNECTIONS_INTERVAL);
//     interval.tick().await;  // skip first tick

//     let mut link_connections: LinkConnections = HashMap::new();
//     loop {
//         tokio::time::sleep(REFRERSH_CONNECTIONS_INTERVAL).await;
//         refresh_connections(&repo, &mut link_connections).await;
//     }
// }

// // TODO: replace with:
// // 1) connect_links_with_autoconnect (call once)
// // 2) conect_disconnect link with channels/pubsub/etc
// async fn refresh_connections(repo: &Arc<db::Repository>, link_connections: &mut LinkConnections) {
//     let response = repo.read_all::<communication::LinkDescription>("link_descriptions").await;
//     match response {
//         Ok(links) => {
//             let mut link_ids: Vec<String> = Vec::new();
//             for link in links {
//                 let link_id = link.id.clone().unwrap();
//                 link_ids.push(link_id.clone());

//                 // Add connections for (new) links
//                 if !link_connections.contains_key(&link_id) {
//                     link_connections.insert(link_id.clone(), create_connection(&link));
//                 }

//                 // Update connection status for link connections
//                 let connection = link_connections.get_mut(&link_id).unwrap();
//                 if link.enabled && !connection.is_connected() {
//                     if let Err(err) = connection.connect().await {
//                         println!("Connect error: {}", err.to_string());
//                     }
//                 } else if !link.enabled && connection.is_connected() {
//                     if let Err(err) = connection.disconnect().await {
//                         println!("Disconnect error: {}", err.to_string());
//                     }
//                 }
//             }

//             for (link_id, connection) in link_connections.iter_mut() {
//                 // Disconnect removed links
//                 if !link_ids.contains(&link_id) {
//                     if let Err(err) = connection.disconnect().await {
//                         println!("Disconnect (on remove) error: {}", err.to_string());
//                     }
//                 // Update link status
//                 } else {
//                     let status = communication::LinkStatus {
//                         id: link_id.clone(),
//                         is_connected: connection.is_connected(),
//                         is_online: connection.is_online()
//                     };

//                     let result = repo.create_or_update("link_statuses", &status).await;
//                     if let Err(err) = result {
//                         println!("Save connection status error: {}", err.to_string());
//                     }
//                     println!("-----> {:?}", status);
//                 }
//             }

//             // Remove connections for deleted links
//             link_connections.retain(|link_id, _| link_ids.contains(&link_id));
//         },
//         Err(err) => panic!("Repository error: {}", err.to_string()),
//     }
// }

// fn create_connection(link: &communication::LinkDescription) -> LickConnection {
//     match &link.protocol {
//         communication::LinkProtocol::Mavlink { link_type, protocol_version } => {
//             Box::new(mavlink::connection::MavlinkConnection::new(link_type, protocol_version))
//         },
//         // NOTE: other protocols should be handled here
//     }
// }

