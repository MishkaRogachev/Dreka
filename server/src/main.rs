
#[tokio::main]
async fn main() -> std::io::Result<()> {
    return dreka_server::start().await;
}