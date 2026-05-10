//! This crate implements the program's Command Line Interface.

use indicatif::{MultiProgress, ProgressBar, ProgressFinish, ProgressStyle};
use log::debug;

use crate::{args::Job, reporter::CliReporter};

pub mod args;

mod generate;
mod reporter;
mod scan;

/// Runs the CLI frontend.
pub async fn run(args: args::Args) -> shared::Result<()> {
    debug!("Started CLI frontend");

    let result = match args.job {
        Job::Gen {
            extracted_dir,
            ref_dir,
        } => generate::run(extracted_dir, ref_dir).await,
        Job::Scan {
            content_dir,
            ref_dir,
            dump,
        } => scan::run(content_dir, ref_dir, dump).await,
    };

    debug!("Finished CLI frontend");
    result
}

// NOTE: Used by some of the modules
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
