use async_trait::async_trait;

#[async_trait]
pub trait IConnection {
    async fn connect(&mut self) -> anyhow::Result<bool>;
    async fn disconnect(&mut self) -> anyhow::Result<bool>;

    async fn is_connected(&self) -> bool;
    async fn is_online(&self) -> bool;

    async fn bytes_received(&self) -> usize;
    async fn bytes_sent(&self) -> usize;
}
