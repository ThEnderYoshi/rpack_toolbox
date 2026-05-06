//! This crate implements the program's Command Line Interface.

mod args;
mod reporter;

use clap::Parser;
use clio::ClioPath;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressFinish, ProgressStyle};
use log::{debug, info, warn};
use tabled::{
    Table, Tabled,
    settings::{Alignment, Style, object::Columns},
};

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
        Job::Scan {
            content_dir,
            ref_dir,
            dump,
        } => scan(content_dir, ref_dir, dump).await,
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

#[derive(Tabled)]
struct ScanTable {
    #[tabled(rename = "Asset")]
    asset_kind: String,
    #[tabled(rename = "Replaced")]
    replaced: u16,
    #[tabled(rename = "Total")]
    total: u16,
}

async fn scan(
    content_dir: ClioPath,
    ref_dir: ClioPath,
    dump: Option<clio::Output>,
) -> shared::Result<()> {
    let mp = MultiProgress::new();

    let reporters = [
        get_progress_bar(&mp, "Fonts"),
        get_progress_bar(&mp, "Images"),
        get_progress_bar(&mp, "Musics"),
        get_progress_bar(&mp, "Sounds"),
        get_progress_bar(&mp, "Translations"),
    ];

    let results = shared::scan::run_job(
        content_dir.to_path_buf(),
        ref_dir.to_path_buf(),
        reporters,
        dump,
    )
    .await?;

    let mut result_table = vec![];
    let mut problems = vec![];

    let kinds = {
        let mut k: Vec<_> = results.keys().collect();
        k.sort_unstable();
        k
    };

    for kind in kinds {
        let data = &results[kind];

        result_table.push(ScanTable {
            asset_kind: kind.to_string(),
            replaced: data.replaced,
            total: data.total,
        });

        problems.extend(data.problems.iter().map(|p| (kind, p)));
    }

    let mut result_table = Table::new(result_table);
    result_table
        .with(Style::sharp())
        .modify(Columns::new(1..=2), Alignment::right());

    info!("Scan results:\n{result_table}");

    if problems.is_empty() || !log::log_enabled!(log::Level::Warn) {
        return Ok(());
    }

    warn!("Found {} problem(s):", problems.len());

    for (kind, problem) in problems {
        eprintln!(
            "{:>20} {}:\t{}",
            kind.to_string().bright_black().bold(),
            problem.path.display().to_string().red().bold(),
            problem.msg,
        );
    }

    Ok(())
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
