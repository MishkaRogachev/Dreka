

pub trait Connection {
    fn connect(&self) -> std::io::Result<()>;
    fn disconnect(&self) -> std::io::Result<()>;

    // TODO: getter for connection statistics
}