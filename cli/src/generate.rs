//! Defines the CLI logic for the Generate tool.

use clio::ClioPath;
use indicatif::MultiProgress;

pub async fn run(extracted_dir: ClioPath, ref_dir: ClioPath) -> shared::Result<()> {
    let mp = MultiProgress::new();

    let reporters = [
        super::get_progress_bar(&mp, "Fonts"),
        super::get_progress_bar(&mp, "Images"),
        super::get_progress_bar(&mp, "Musics"),
        super::get_progress_bar(&mp, "Sounds"),
        super::get_progress_bar(&mp, "Translations"),
    ];

    shared::generate::run_job(
        extracted_dir.to_path_buf(),
        ref_dir.to_path_buf(),
        reporters,
    )
    .await
}
