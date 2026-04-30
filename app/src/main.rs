#[tokio::main]
async fn main() -> shared::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    cli::run().await
}
