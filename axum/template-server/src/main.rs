use template_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    template_server::run().await
}