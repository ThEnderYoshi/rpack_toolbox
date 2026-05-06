//! Defines the CLI logic for the Scan tool.

use clio::ClioPath;
use colored::Colorize;
use indicatif::MultiProgress;
use log::{info, warn};
use tabled::{
    Table, Tabled,
    settings::{Alignment, Style, object::Columns},
};

#[derive(Tabled)]
struct ScanTable {
    #[tabled(rename = "Asset")]
    asset_kind: String,

    #[tabled(rename = "Replaced")]
    replaced: u16,

    #[tabled(rename = "Total")]
    total: u16,
}

pub async fn run(
    content_dir: ClioPath,
    ref_dir: ClioPath,
    dump: Option<clio::Output>,
) -> shared::Result<()> {
    let mp = MultiProgress::new();

    let reporters = [
        super::get_progress_bar(&mp, "Fonts"),
        super::get_progress_bar(&mp, "Images"),
        super::get_progress_bar(&mp, "Musics"),
        super::get_progress_bar(&mp, "Sounds"),
        super::get_progress_bar(&mp, "Translations"),
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
