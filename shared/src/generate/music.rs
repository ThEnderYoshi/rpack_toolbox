//! Responsible for generating the music reference file.
//!
//! See [`generate`] for more information.

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
    let task_size = WalkDir::new(&extracted_dir)
        .max_depth(1)
        .into_iter()
        .count();

    reporter.report_init(task_size);
    let (sender, receiver) = mpsc::channel();

    WalkDir::new(&extracted_dir)
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
    ref_files::write_flat_ref(&out_dir.join("music.txt"), ref_files::FORMAT_MUS, flat)?;
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

    if path.extension() != Some(OsStr::new("wav")) {
        sender.send(None).unwrap();
        return Ok(());
    }

    let Some(name) = ref_files::get_flat_value(&path) else {
        sender.send(None).unwrap();
        return Ok(());
    };

    let Some((number, _)) = name.split_once(' ') else {
        sender.send(None).unwrap();
        return Ok(());
    };

    // Parse into int then re-serialize to remove leading zeros
    // Fall back to raw str if it can't be parsed
    let number = match number.parse::<u16>() {
        Ok(n) => n.to_string(),
        Err(_) => number.to_string(),
    };

    let name = format!("Music_{number}");
    sender.send(Some(name)).unwrap();
    Ok(())
}
