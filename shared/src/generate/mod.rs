//! This module defines the "Generate" tool.
//!
//! See [`run_job`] for more information.

use std::{fs, path::PathBuf};

use log::info;

use crate::{reporter::Reporter, sync};

mod fonts;
mod images;
mod music;
mod sounds;
mod translations;

/// Executes the "Generate" tool.
///
/// `extracted_dir` should be a path to where the files extracted with
/// `TConvert` and `TerrariaLocalizationPacker` were dumped to.
pub async fn run_job(
    extracted_dir: PathBuf,
    out_dir: PathBuf,
    reporters: [impl Reporter + Send + 'static; 5],
) -> crate::Result<()> {
    info!(
        "Generating reference files from assets at `{}`",
        extracted_dir.display()
    );

    // Setup output dir
    if !out_dir.is_dir() {
        fs::create_dir_all(&out_dir)?;
    }

    // Run generators
    let [r_fnt, r_img, r_mus, r_snd, r_txt] = reporters;

    let ex_dir_clone = extracted_dir.clone();
    let out_dir_clone = out_dir.clone();
    let fnt = tokio::spawn(async move { fonts::generate(r_fnt, ex_dir_clone, out_dir_clone) });

    let ex_dir_clone = extracted_dir.clone();
    let out_dir_clone = out_dir.clone();
    let img = tokio::spawn(async move { images::generate(r_img, ex_dir_clone, out_dir_clone) });

    let ex_dir_clone = extracted_dir.clone();
    let out_dir_clone = out_dir.clone();
    let mus = tokio::spawn(async move { music::generate(r_mus, ex_dir_clone, out_dir_clone) });

    let ex_dir_clone = extracted_dir.clone();
    let out_dir_clone = out_dir.clone();
    let snd = tokio::spawn(async move { sounds::generate(r_snd, ex_dir_clone, out_dir_clone) });

    let out_dir_clone = out_dir.clone();

    let txt =
        tokio::spawn(async move { translations::generate(r_txt, extracted_dir, out_dir_clone) });

    tokio::try_join!(
        sync::flatten(fnt),
        sync::flatten(img),
        sync::flatten(mus),
        sync::flatten(snd),
        sync::flatten(txt),
    )?;

    info!("Files written to `{}`", out_dir.display());
    Ok(())
}

// async fn flatten<T>(handle: JoinHandle<crate::Result<T>>) -> crate::Result<T> {
//     handle.await?
// }
