//! Responsible for generating the sound reference files.
//!
//! See [`generate`] for more information.

use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use crate::{
    ref_files,
    reporter::{Reporter, Update},
};

/// Generates the sound reference files.
pub fn generate(
    mut reporter: impl Reporter,
    extracted_dir: PathBuf,
    out_dir: PathBuf,
) -> crate::Result<()> {
    let snd_dir = extracted_dir.join("Sounds");
    let task_size = WalkDir::new(&snd_dir).into_iter().count();
    reporter.report_init(task_size);

    let (sender, receiver) = mpsc::channel();

    WalkDir::new(&snd_dir)
        .into_iter()
        .par_bridge()
        .try_for_each_with((sender, snd_dir), process_entry)?;

    let mut tree = HashMap::new();

    for update in receiver.iter() {
        if let Some((dir, file)) = update {
            tree.entry(dir).or_insert_with(HashSet::new).insert(file);
        }

        reporter.report_update(Update::Processed(1));
    }

    reporter.report_update(Update::Message("Writing reference file...".into()));
    ref_files::write_tree_ref(&out_dir.join("sounds.txt"), ref_files::FORMAT_SND, tree)?;
    reporter.report_completed(None);
    Ok(())
}

fn process_entry(
    (sender, root): &mut (Sender<Option<(String, String)>>, PathBuf),
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

    let Some(key_value) = ref_files::get_tree_key_value(root, &path) else {
        sender.send(None).unwrap();
        return Ok(());
    };

    sender.send(Some(key_value)).unwrap();
    // let size = ref_files::get_size_string(imagesize::size(&path)?);
    // let file = format!("{file} {size}");
    // sender.send(Some((dir, file))).unwrap();
    Ok(())
}
