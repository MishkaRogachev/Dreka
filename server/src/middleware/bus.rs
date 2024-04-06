use tokio::sync::broadcast;

const CHANNEL_CAPACITY: usize = 2000;

#[derive(Clone)]
pub struct EventBus<T: Send + std::clone::Clone> {
    sender: broadcast::Sender<T>,
}

impl<T: Send + std::clone::Clone> EventBus<T> {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<T> {
        self.sender.subscribe()
    }

    pub fn publish(&self, event: T) -> anyhow::Result<()> {
        if self.sender.receiver_count() == 0 {
            return Ok(()); // No subscribers, no need to send the event
        }

        if let Err(err) = self.sender.send(event) {
            return Err(anyhow::Error::msg(err.to_string()));
        }
        Ok(())
    }
}
