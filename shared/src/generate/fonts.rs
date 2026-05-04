//! Responsible for generating the font reference file.
//!
//! See [`generate`] for more info.

use std::{
    ffi::OsStr,
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::{DirEntry, WalkDir};

use crate::{
    ref_files::{self, FlatRef},
    reporter::{Reporter, Update},
};

pub fn generate(
    mut reporter: impl Reporter,
    extracted_dir: PathBuf,
    out_dir: PathBuf,
) -> crate::Result<()> {
    let fnt_dir = extracted_dir.join("Fonts");

    let task_size = WalkDir::new(&fnt_dir).max_depth(1).into_iter().count();

    reporter.report_init(task_size);
    let (sender, receiver) = mpsc::channel();

    WalkDir::new(&fnt_dir)
        .into_iter()
        .par_bridge()
        .try_for_each_with(sender, process_entry)?;

    let mut flat = FlatRef::new();

    for update in receiver.iter() {
        if let Some(file) = update {
            flat.insert(file);
        }

        reporter.report_update(Update::Processed(1));
    }

    reporter.report_update(Update::Message("Writing reference file...".into()));
    ref_files::write_flat_ref(&out_dir.join("fonts.txt"), ref_files::FORMAT_FNT, flat)?;
    reporter.report_completed(None);
    Ok(())
}

fn process_entry(
    sender: &mut Sender<Option<String>>,
    entry: walkdir::Result<DirEntry>,
) -> crate::Result<()> {
    let entry = entry?;

    if !entry.file_type().is_file() {
        sender.send(None).unwrap();
        return Ok(());
    }

    let path = entry.into_path();

    if path.extension() != Some(OsStr::new("png")) {
        sender.send(None).unwrap();
        return Ok(());
    }

    let path = match path.file_stem() {
        Some(p) => p.to_string_lossy(),
        None => {
            sender.send(None).unwrap();
            return Ok(());
        }
    };

    let maybe_name = path.strip_suffix("_0").map(|s| s.to_string());
    sender.send(maybe_name).unwrap();
    Ok(())
}
