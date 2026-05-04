//! This module defines the "Scan" tool.
//!
//! See [`run_job`] for more information.

use std::{collections::HashMap, fmt::Display, path::PathBuf};

use log::info;
use serde::Deserialize;

use crate::{reporter::Reporter, rpack_data::AssetKind, sync};

mod fonts;
mod images;
mod music;
mod sounds;
mod translations;

/// The data collected by one of the scanners.
#[derive(Deserialize)]
pub struct ScanData {
    /// The amount of valid assets of this kind in the resource pack.
    pub replaced: u16,
    /// The total amount of assets of this kind in the game, as defined in the
    /// reference files.
    pub total: u16,
    /// Details any invalid assets encountered by the scanner.
    pub problems: Vec<InvalidAsset>,
}

impl ScanData {
    /// Creates a new [`ScanData`] structure.
    pub fn new(total: u16) -> Self {
        Self {
            replaced: 0,
            total,
            problems: vec![],
        }
    }
}

/// Represents an invalid asset encountered by a scanner.
#[derive(Deserialize)]
pub struct InvalidAsset {
    /// The path to the asset, relative to the resource pack's root.
    pub path: PathBuf,
    /// Explains why the asset is invalid.
    pub msg: String,
}

impl InvalidAsset {
    /// Creates a new [`InvalidAsset`].
    pub fn new(path: impl Into<PathBuf>, msg: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            msg: msg.into(),
        }
    }
}

impl Display for InvalidAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "At `{}`:\t{}", self.path.display(), self.msg)
    }
}

enum ScanResult {
    Valid,
    Invalid(InvalidAsset),
    Skipped,
}

impl From<InvalidAsset> for ScanResult {
    fn from(value: InvalidAsset) -> Self {
        ScanResult::Invalid(value)
    }
}

/// Executes the "Generate" tool.
pub async fn run_job(
    pack_root: PathBuf,
    ref_dir: PathBuf,
    reporters: [impl Reporter + Send + 'static; 5],
) -> crate::Result<HashMap<AssetKind, ScanData>> {
    info!(
        "Scanning resource pack `{}`...",
        pack_root.file_name().map_or_else(
            || pack_root.display().to_string(),
            |n| n.display().to_string(),
        ),
    );

    let [r_fnt, r_img, r_mus, r_snd, r_txt] = reporters;
    let content_dir = pack_root.join("Content");

    let content_dir_clone = content_dir.clone();
    let ref_dir_clone = ref_dir.clone();
    let fnt = tokio::spawn(async move { fonts::scan(r_fnt, content_dir_clone, ref_dir_clone) });

    let content_dir_clone = content_dir.clone();
    let ref_dir_clone = ref_dir.clone();
    let img = tokio::spawn(async move { images::scan(r_img, content_dir_clone, ref_dir_clone) });

    let content_dir_clone = content_dir.clone();
    let ref_dir_clone = ref_dir.clone();
    let mus = tokio::spawn(async move { music::scan(r_mus, content_dir_clone, ref_dir_clone) });

    let content_dir_clone = content_dir.clone();
    let ref_dir_clone = ref_dir.clone();
    let snd = tokio::spawn(async move { sounds::scan(r_snd, content_dir_clone, ref_dir_clone) });

    let content_dir_clone = content_dir.clone();
    let ref_dir_clone = ref_dir.clone();

    let txt =
        tokio::spawn(async move { translations::scan(r_txt, content_dir_clone, ref_dir_clone) });

    let (fnt, img, mus, snd, txt) = tokio::try_join!(
        sync::flatten(fnt),
        sync::flatten(img),
        sync::flatten(mus),
        sync::flatten(snd),
        sync::flatten(txt),
    )?;

    let mut results = HashMap::new();

    if let Some(result) = fnt {
        results.insert(AssetKind::Font, result);
    }

    if let Some(result) = img {
        results.insert(AssetKind::Image, result);
    }

    if let Some(result) = mus {
        results.insert(AssetKind::Music, result);
    }

    if let Some(result) = snd {
        results.insert(AssetKind::Sound, result);
    }

    for (lang, result) in txt {
        results.insert(AssetKind::Translation(lang), result);
    }

    info!("Scan complete");
    Ok(results)
}
