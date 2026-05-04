//! The `shared` crate contains the base logic shared between all other crates.

pub mod error;
pub mod generate;
pub mod reporter;
pub mod rpack_data;
pub mod scan;

mod ref_files;
mod sync;

pub use error::{Error, Result};
