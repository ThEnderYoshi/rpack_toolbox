//! Defines the logic for scanning translation files.

use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        mpsc::{self, Sender},
    },
};

use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::Deserialize;
use walkdir::{DirEntry, WalkDir};

use crate::{
    ref_files,
    reporter::{Reporter, Update},
    rpack_data::Language,
    scan::{InvalidAsset, ScanData, ScanResult},
};

enum ProcessResult {
    FileFinished,
    InvalidFile(InvalidAsset),
    Entry { lang: Language, result: ScanResult },
}

struct TranslationRef {
    data: HashSet<String>,
    root: PathBuf,
    found: HashMap<Language, Mutex<HashMap<String, PathBuf>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CsvRecord {
    key: String,
    #[serde(rename = "Translation")]
    _translation: String,
}

impl TranslationRef {
    fn load(path: &Path, root: PathBuf) -> crate::Result<Self> {
        let raw = ref_files::read_tree_ref(path, ref_files::FORMAT_TXT)?;
        let mut data = HashSet::new();

        for (category, keys) in raw {
            for key in keys {
                data.insert(if category.is_empty() {
                    key
                } else {
                    format!("{category}.{key}")
                });
            }
        }

        let found = HashMap::from([
            (Language::English, Mutex::new(HashMap::new())),
            (Language::German, Mutex::new(HashMap::new())),
            (Language::Italian, Mutex::new(HashMap::new())),
            (Language::French, Mutex::new(HashMap::new())),
            (Language::Spanish, Mutex::new(HashMap::new())),
            (Language::Russian, Mutex::new(HashMap::new())),
            (Language::ChineseSimplified, Mutex::new(HashMap::new())),
            (Language::ChineseTraditional, Mutex::new(HashMap::new())),
            (Language::Portuguese, Mutex::new(HashMap::new())),
            (Language::Polish, Mutex::new(HashMap::new())),
            (Language::Japanese, Mutex::new(HashMap::new())),
            (Language::Korean, Mutex::new(HashMap::new())),
        ]);

        Ok(Self { data, root, found })
    }

    fn validate_csv(&self, sender: &Sender<ProcessResult>, path: &Path) -> crate::Result<()> {
        let rel_path = path.strip_prefix(&self.root)?;

        let Some(name) = path.file_name().map(|n| n.to_string_lossy()) else {
            sender
                .send(ProcessResult::InvalidFile(InvalidAsset::new(
                    rel_path,
                    "invalid file name",
                )))
                .unwrap();

            return Ok(());
        };

        let Some(lang) = Language::from_start_of_string(&name) else {
            sender
                .send(ProcessResult::InvalidFile(InvalidAsset::new(
                    rel_path,
                    "no valid language code",
                )))
                .unwrap();

            return Ok(());
        };

        let mut reader = csv::Reader::from_path(path)?;

        for record in reader.deserialize() {
            let CsvRecord { key, .. } = record?;

            // We treat keys starting with # as comments, so let's skip them
            if key.starts_with('#') {
                sender
                    .send(ProcessResult::Entry {
                        lang,
                        result: ScanResult::Skipped,
                    })
                    .unwrap();

                continue;
            }

            let result = if !self.data.contains(&key) {
                InvalidAsset::new(rel_path, format!("invalid key: {key}")).into()
            } else {
                let mut found = self.found[&lang].lock().unwrap();

                if let Some(also) = found.get(&key) {
                    InvalidAsset::new(
                        rel_path,
                        format!("duplicate key: {key} (also in {})", also.display()),
                    )
                    .into()
                } else {
                    found.insert(key, rel_path.to_path_buf());
                    ScanResult::Valid
                }
            };

            // let result = if self.data.contains(&key) {
            //     ScanResult::Valid
            // } else {
            //     InvalidAsset::new(rel_path, format!("invalid key: {}", key)).into()
            // };

            sender.send(ProcessResult::Entry { lang, result }).unwrap();
        }

        sender.send(ProcessResult::FileFinished).unwrap();
        Ok(())
    }
}

pub fn scan(
    mut reporter: impl Reporter,
    content_dir: PathBuf,
    ref_dir: PathBuf,
) -> crate::Result<HashMap<Option<Language>, ScanData>> {
    let txt_dir = content_dir.join("Localization");

    if !txt_dir.is_dir() {
        reporter.report_init(0);
        reporter.report_completed(Some("`Localization` folder not found. Skipped.".into()));
        return Ok(HashMap::new());
    }

    let txt_ref = ref_dir.join("translations.txt");
    let txt_ref = Arc::new(TranslationRef::load(&txt_ref, txt_dir.clone())?);

    let task_size = WalkDir::new(&txt_dir).into_iter().count();
    reporter.report_init(task_size);

    let asset_count = txt_ref.data.len() as u16;
    let (sender, receiver) = mpsc::channel();

    WalkDir::new(txt_dir)
        .into_iter()
        .par_bridge()
        .try_for_each_with((sender, txt_ref), process_entry)?;

    let mut data = HashMap::new();

    for result in receiver.iter() {
        match result {
            ProcessResult::FileFinished => {}
            ProcessResult::InvalidFile(i) => data
                .entry(None)
                .or_insert_with(|| ScanData::new(asset_count))
                .problems
                .push(i),
            ProcessResult::Entry { lang, result } => {
                match result {
                    ScanResult::Valid => {
                        data.entry(Some(lang))
                            .or_insert_with(|| ScanData::new(asset_count))
                            .replaced += 1;
                    }
                    ScanResult::Skipped => {}
                    ScanResult::Invalid(i) => data
                        .entry(Some(lang))
                        .or_insert_with(|| ScanData::new(asset_count))
                        .problems
                        .push(i),
                }

                continue; // Don't report update
            }
        }

        reporter.report_update(Update::Processed(1))
    }

    reporter.report_completed(None);
    Ok(data)
}

fn process_entry(
    (sender, txt_ref): &mut (Sender<ProcessResult>, Arc<TranslationRef>),
    entry: walkdir::Result<DirEntry>,
) -> crate::Result<()> {
    let entry = entry?;

    if !entry.file_type().is_file() {
        sender.send(ProcessResult::FileFinished).unwrap();
        return Ok(());
    }

    let path = entry.into_path();
    let ext = path.extension();

    if ext == Some(OsStr::new("csv")) {
        txt_ref.validate_csv(sender, &path)
    } else if ext == Some(OsStr::new("json")) {
        sender
            .send(ProcessResult::InvalidFile(InvalidAsset::new(
                path,
                "json files not yet supported",
            )))
            .unwrap();

        Ok(())
    } else {
        sender
            .send(ProcessResult::InvalidFile(InvalidAsset::new(
                path,
                "not a csv or json file",
            )))
            .unwrap();

        Ok(())
    }
}
