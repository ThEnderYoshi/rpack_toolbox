//! The `shared` crate contains the base logic shared between all other crates.

pub mod error;
pub mod generate;
pub mod reporter;

mod ref_files;

pub use error::{Error, Result};
