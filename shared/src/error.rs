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

    /// Wrapper for [`imagesize::ImageError`].
    #[error("image error: {0}")]
    ImageSize(#[from] imagesize::ImageError),

    /// Wrapper for [`std::io::Error`].
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Wrapper for [`json5::Error`].
    #[error("json error: {0}")]
    Json5(#[from] json5::Error),

    /// Wrapper for [`tokio::task::JoinError`].
    #[error("async error: {0}")]
    TokioJoin(#[from] tokio::task::JoinError),

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
}
