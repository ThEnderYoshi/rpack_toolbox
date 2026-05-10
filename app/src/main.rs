//! This crate implements the main binary of the program.

#[tokio::main]
async fn main() -> shared::Result<()> {
    let result = run_cli().await;

    if let Err(e) = &result {
        log::error!("{e}");
    }

    result
}

async fn run_cli() -> shared::Result<()> {
    let args = cli::args::parse();
    init_logger(args.verbosity.log_level_filter());
    cli::run(args).await
}

fn init_logger(level: log::LevelFilter) {
    pretty_env_logger::formatted_builder()
        .filter(None, level)
        .init();
}
