use std::{sync::Arc, collections::HashMap};
use tokio::{sync::broadcast, time};

use crate::{datasource::db, models::{self, communication::{self, LinkDescription, LinkStatus}, events::ClentEvent}};
use super::{default_links::create_dafault_links, traits, mavlink::connection::MavlinkConnection};

type LickConnection = Box<dyn traits::IConnection + Send + Sync>;
type LinkConnections = HashMap<String, LickConnection>;

const CHECK_CONNECTIONS_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(100);

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
            let status = LinkStatus::default_for_id(&link_id);
            let result = self.repository.create_or_update("link_statuses", &status).await;
            if let Err(err) = result {
                return Err(ServiceError::Db(err));
            }

            // Autoconect specified one
            if link.autoconnect {
                let result = self.connect_link(&link_id).await;
                if let Err(err) = result {
                    println!("Autoconnect link event error: {}", err);
                }
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

            let mut links_to_disconnect = Vec::new();
            // Collect link statistics
            for (link_id, connection) in self.link_connections.iter_mut() {
                let status: LinkStatus;
                if connection.is_healthy().await {
                    status = create_connection_status(link_id, connection).await;
                } else {
                    if let Err(err) = connection.disconnect().await {
                        println!("Error disconnecting link: {}", err);
                    }
                    links_to_disconnect.push(link_id.to_owned());
                    status = LinkStatus::default_for_id(&link_id);
                }

                let result = self.repository.create_or_update("link_statuses", &status).await;
                if let Err(err) = result {
                    println!("Link update status error: {}", err);
                }
            }

            // Remove faulted connections
            for link_id in links_to_disconnect {
                self.link_connections.remove(&link_id);
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
        if self.link_connections.contains_key(link_id) {
            println!("Link is already connected {}", link_id);
            return Ok(());
        }

        let link = self.repository.read("link_descriptions", link_id).await;
        if let Err(err) = link {
            return Err(ServiceError::Db(err));
        }
        let mut connection = create_connection(self.repository.clone(),&link.unwrap());
        if let Err(err) = connection.connect().await {
            return Err(ServiceError::Connection(err));
        }

        let status = create_connection_status(&link_id, &connection).await;
        let result = self.repository.create_or_update("link_statuses", &status).await;
        if let Err(err) = result {
            return Err(ServiceError::Db(err));
        }
        self.link_connections.insert(link_id.to_string(), connection);
        Ok(())
    }

    async fn disconnect_link(&mut self, link_id: &str) -> Result<(), ServiceError> {
        if !self.link_connections.contains_key(link_id) {
            println!("No connection found for link {}", link_id);
            return Ok(());
        }
        
        let mut connection = self.link_connections.remove(link_id).unwrap();
        if let Err(err) = connection.disconnect().await {
            return Err(ServiceError::Connection(err))
        }

        let status = LinkStatus::default_for_id(&link_id);
        let result = self.repository.create_or_update("link_statuses", &status).await;
        if let Err(err) = result {
            return Err(ServiceError::Db(err));
        }
        Ok(())
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
        }
    }
}

fn create_connection(repository: Arc<db::Repository>, link: &communication::LinkDescription) -> LickConnection {
    match &link.protocol {
        communication::LinkProtocol::Mavlink { link_type, protocol_version } => {
            Box::new(MavlinkConnection::new(repository, link_type, protocol_version))
        },
        // NOTE: other protocols should be handled here
    }
}

async fn create_connection_status(link_id: &str, connection: &LickConnection) -> communication::LinkStatus {
    communication::LinkStatus {
        id: link_id.into(),
        is_connected: true, // NOTE: connection.is_connected may be wrong in this context
        is_online: connection.is_online().await,
        bytes_received: connection.bytes_received().await,
        bytes_sent: connection.bytes_sent().await,
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
