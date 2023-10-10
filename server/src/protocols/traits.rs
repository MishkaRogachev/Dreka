use async_trait::async_trait;

#[async_trait]
pub trait IConnection {
    async fn connect(&mut self) -> std::io::Result<()>;
    async fn disconnect(&mut self) -> std::io::Result<()>;

    fn is_connected(&self) -> bool;
    fn is_online(&self) -> bool;

    // TODO: getter for connection statistics
}
