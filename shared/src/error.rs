//! This module defines the crate's [`Error`] type, as well as a convenience
//! [`Result`] alias.
//!
//! See their respective documentations for more information.

use std::path::{Path, PathBuf};

/// Alias of [`Result`][std::result::Result] where `E` is this crate's
/// [`Error`] type.
///
/// This type is re-exported by [`shared`][crate].
pub type Result<T> = std::result::Result<T, Error>;

/// This crate's error type.
///
/// This type is re-exported by [`shared`][crate].
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Used when something goes wrong while parsing a reference file.
    ///
    /// This variant is usually created through one of [`Error`]'s constructors.
    #[error("invalid reference file `{}`: {message}", path.display())]
    BadRefFile { path: PathBuf, message: String },

    /// Used when parsing an [`IVec2`][crate::jigsaw::cfg::IVec2] fails.
    #[error("could not parse 2d vector '{0}'")]
    ParseIVec2(String),

    /// Used when parsing a [`PieceCfg`][crate::jigsaw::cfg::PieceCfg] fails.
    #[error("could not parse jigsaw piece '{0}'")]
    ParsePiece(String),

    /// Wrapper for [`csv::Error`].
    #[error("csv error: {0}")]
    Csv(#[from] csv::Error),

    /// Wrapper for [`image::ImageError`].
    #[error("image error: {0}")]
    Image(#[from] image::ImageError),

    /// Wrapper for [`imagesize::ImageError`].
    #[error("image error: {0}")]
    ImageSize(#[from] imagesize::ImageError),

    /// Wrapper for [`std::io::Error`].
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Wrapper for [`serde_json::Error`].
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Wrapper for [`json5::Error`].
    #[error("json error: {0}")]
    Json5(#[from] json5::Error),

    /// Wrapper for [`std::path::StripPrefixError`].
    #[error("path error: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),

    /// Wrapper for [`tokio::task::JoinError`].
    #[error("async error: {0}")]
    TokioJoin(#[from] tokio::task::JoinError),

    /// Wrapper for [`toml::de::Error`].
    #[error("toml parse error: {0}")]
    TomlDe(#[from] toml::de::Error),

    /// Wrapper for [`walkdir::Error`].
    #[error("walkdir error: {0}")]
    WalkDir(#[from] walkdir::Error),
}

impl Error {
    /// Creates an [`Error::BadRefFile`] that explains it has no format version.
    pub fn no_format_version(path: &Path) -> Self {
        Self::bad_ref_file(path, "no format version")
    }

    /// Creates an [`Error::BadRefFile`] that explains its format version
    /// is incorrect.
    pub fn bad_format_version(path: &Path, expected: u8, actual: u8) -> Self {
        Self::bad_ref_file(
            path,
            format!("unsupported format version {actual} (expected {expected})"),
        )
    }

    /// Creates an [`Error::BadRefFile`].
    pub fn bad_ref_file(path: &Path, message: impl Into<String>) -> Self {
        Self::BadRefFile {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Creates an [`Error::ParseIVec2`].
    pub fn parse_ivec2(raw: &str) -> Self {
        Self::ParseIVec2(raw.into())
    }

    /// Creates an [`Error::ParsePiece`].
    pub fn parse_piece(raw: &str) -> Self {
        Self::ParsePiece(raw.into())
    }
}
