//! Defines the CLI logic for the Jigsaw tool.

use clio::ClioPath;

pub async fn run(config: ClioPath, input: ClioPath, output: ClioPath) -> shared::Result<()> {
    let config = config.to_path_buf();
    let input = input.to_path_buf();
    let output = output.to_path_buf();
    shared::jigsaw::run_job(config, input, output).await
}
