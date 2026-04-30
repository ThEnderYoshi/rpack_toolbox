//! This crate implements the program's Command Line Interface.

mod args;
mod reporter;

use clap::Parser;
use clio::ClioPath;
use indicatif::{MultiProgress, ProgressBar, ProgressFinish, ProgressStyle};
use log::debug;

use crate::{args::Job, reporter::CliReporter};

/// Runs the CLI frontent.
pub async fn run() -> shared::Result<()> {
    debug!("Started CLI frontend");
    let args = args::Args::parse();

    match args.job {
        Job::Gen {
            extracted_dir,
            ref_dir,
        } => generate(extracted_dir, ref_dir).await,
    }
}

async fn generate(extracted_dir: ClioPath, ref_dir: ClioPath) -> shared::Result<()> {
    let mp = MultiProgress::new();

    let reporters = [
        get_progress_bar(&mp, "Fonts"),
        get_progress_bar(&mp, "Images"),
        get_progress_bar(&mp, "Musics"),
        get_progress_bar(&mp, "Sounds"),
        get_progress_bar(&mp, "Translations"),
    ];

    shared::generate::run_job(
        extracted_dir.to_path_buf(),
        ref_dir.to_path_buf(),
        reporters,
    )
    .await
}

fn get_progress_bar(mp: &MultiProgress, title: &str) -> CliReporter {
    let style =
        ProgressStyle::with_template("[{elapsed:>3}] {bar:40.cyan/blue} {pos:>5}/{len:>5} : {msg}")
            .unwrap()
            .progress_chars("██▒");

    let bar = ProgressBar::new(1)
        .with_style(style)
        .with_message(format!("{title:12} : Reading files..."))
        .with_finish(ProgressFinish::AndLeave);

    CliReporter::new(mp.add(bar), title)
}
