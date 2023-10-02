use async_trait::async_trait;

#[async_trait]
pub trait Connection {
    async fn connect(&mut self) -> std::io::Result<()>;
    async fn disconnect(&mut self) -> std::io::Result<()>;

    fn is_connected(&self) -> bool;

    // TODO: getter for connection statistics
}