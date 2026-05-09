//! This crate represents the final binary of the program.

#[tokio::main]
async fn main() -> shared::Result<()> {
    run_cli().await
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
