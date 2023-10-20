use async_trait::async_trait;

#[derive(Debug)]
pub enum ConnectionError {
    Io(std::io::Error),
}

#[async_trait]
pub trait IConnection {
    async fn connect(&mut self) -> Result<bool, ConnectionError>;
    async fn disconnect(&mut self) -> Result<bool, ConnectionError>;

    fn is_healthy(&self) -> bool;
    fn is_online(&self) -> bool;

    fn bytes_received(&self) -> usize;
    fn bytes_sent(&self) -> usize;
}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConnectionError::Io(err) => write!(f, "{}", err),
        }
    }
}