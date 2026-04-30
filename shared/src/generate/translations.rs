//! Responsible for generating the translation reference files.
//!
//! See [`generate`] for more information.

use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::{DirEntry, WalkDir};

use crate::{
    ref_files::{self, TreeRef},
    reporter::{Reporter, Update},
};

type TranslationFile = HashMap<String, HashMap<String, String>>;

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

    let mut tree = TreeRef::new();

    for update in receiver.iter() {
        if let Some(key_values) = update {
            for (category, key) in key_values {
                tree.entry(category).or_default().insert(key);
            }
        }

        reporter.report_update(Update::Processed(1));
    }

    reporter.report_update(Update::Message("Writing reference file...".into()));

    ref_files::write_tree_ref(
        &out_dir.join("translations.txt"),
        ref_files::FORMAT_TXT,
        tree,
    )?;

    reporter.report_completed(None);
    Ok(())
}

fn process_entry(
    sender: &mut Sender<Option<Vec<(String, String)>>>,
    entry: walkdir::Result<DirEntry>,
) -> crate::Result<()> {
    let entry = entry?;

    if !entry.file_type().is_file() {
        sender.send(None).unwrap();
        return Ok(());
    }

    let path = entry.into_path();

    if path.extension() != Some(OsStr::new("json")) {
        sender.send(None).unwrap();
        return Ok(());
    }

    // NOTE: We use JSON5 instead of JSON here because some of the translation
    // files have trailing commas, which are not valid JSON and, therefore, not
    // supported by serde_json
    let data: TranslationFile = json5::from_str(&fs::read_to_string(path)?)?;
    let mut key_values = vec![];

    for (category, values) in data {
        key_values = values.into_keys().fold(key_values, |mut acc, k| {
            acc.push((category.clone(), k));
            acc
        });
    }

    sender.send(Some(key_values)).unwrap();
    Ok(())
}
