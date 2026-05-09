//! Defines this crate's command line arguments.
//!
//! See [`Args`] for more information.

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use clio::{ClioPath, Output};

/// Alias for [`Args::parse`], so you don't have to import [`clap::Parser`] to
/// parse the command line arguments.
pub fn parse() -> Args {
    Args::parse()
}

/// Multipurpose tool to aid in the creation of Terraria resource packs
///
/// For more information, see <https://github.com/ThEnderYoshi/rpack_toolbox>
#[derive(Parser)]
#[command(name = "rpack_toolbox")]
pub struct Args {
    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,

    #[command(subcommand)]
    pub job: Job,
}

#[derive(Subcommand)]
pub enum Job {
    /// Generates the reference files used by the `scan` tool
    ///
    /// This tool uses the game assets extracted by TConvert and
    /// TerrariaLocalizationPacker to generate the files
    Gen {
        /// Path to where the extracted game assets were dumped to
        #[arg(value_parser = clap::value_parser!(ClioPath).exists().is_dir())]
        extracted_dir: ClioPath,

        /// Path where the reference files will be dumped to
        ///
        /// The dir will be created if it doesn't already exist
        #[arg(value_parser = clap::value_parser!(ClioPath).is_dir())]
        ref_dir: ClioPath,
    },

    /// Scans a resource pack to detect problems and give other useful insight
    ///
    /// The scanner needs the reference files created by the `gen` command
    /// to work.
    ///
    /// The insights given are as follows (more may be added in the future):
    ///
    /// - Count the amount of assets replaced
    ///
    /// - Detect invalid assets
    ///
    /// - Detect duplicate translation keys (takes multiple languages
    ///   into account)
    Scan {
        /// Path to the root dir of the resource pack
        #[arg(value_parser = clap::value_parser!(ClioPath).is_dir().exists())]
        content_dir: ClioPath,

        /// Path to the dir with the reference files created by `gen`
        #[arg(value_parser = clap::value_parser!(ClioPath).is_dir().exists())]
        ref_dir: ClioPath,

        /// If set, the scan data will be dumped to a JSON file at the specified
        /// path, or to stdout if '-' is passed
        ///
        /// TIP: This is the only output of this tool that writes to stdout.
        /// Everything else is written to stderr
        #[arg(long, short, value_parser)]
        dump: Option<Output>,
    },
}
