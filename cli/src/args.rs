//! Defines this crate's command line arguments.
//!
//! See [`Args`] for more information.

use clap::{Parser, Subcommand};
// use clap_verbosity_flag::{InfoLevel, Verbosity};
use clio::ClioPath;

#[derive(Parser)]
pub struct Args {
    // #[command(flatten)]
    // pub verbosity: Verbosity<InfoLevel>,
    #[command(subcommand)]
    pub job: Job,
}

#[derive(Subcommand)]
pub enum Job {
    /// Generates the reference files used by the `scan` tool.
    ///
    /// This tool uses the game assets extracted by TConvert and
    /// TerrariaLocalizationPacker to generate the files.
    Gen {
        /// Path to where the extracted game assets were dumped to.
        #[arg(value_parser = clap::value_parser!(ClioPath).exists().is_dir())]
        extracted_dir: ClioPath,

        /// Path where the reference files will be dumped to.
        ///
        /// The dir is recursively created if it doesn't already exist.
        #[arg(value_parser = clap::value_parser!(ClioPath).is_dir())]
        ref_dir: ClioPath,
    },
}
