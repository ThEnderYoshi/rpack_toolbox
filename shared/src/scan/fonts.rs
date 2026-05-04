//! Defines the logic for scanning fonts.

use std::{
    collections::HashSet,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::{
        Arc,
        mpsc::{self, Sender},
    },
};

use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::{DirEntry, WalkDir};

use crate::{
    ref_files,
    reporter::{Reporter, Update},
    scan::{InvalidAsset, ScanData, ScanResult},
};

struct FontRef {
    data: HashSet<PathBuf>,
    root: PathBuf,
}

impl FontRef {
    fn load(path: &Path, root: PathBuf) -> crate::Result<Self> {
        let raw = ref_files::read_flat_ref(path, ref_files::FORMAT_FNT)?;
        let data = raw.into_iter().map(PathBuf::from).collect();
        Ok(Self { data, root })
    }

    fn validate(&self, path: &Path) -> crate::Result<ScanResult> {
        let ref_path = path.strip_prefix(&self.root)?.with_extension("");

        if self.data.contains(&ref_path) {
            Ok(ScanResult::Valid)
        } else {
            Ok(InvalidAsset::new(ref_path, "invalid file name").into())
        }
    }
}

pub fn scan(
    mut reporter: impl Reporter,
    content_dir: PathBuf,
    ref_dir: PathBuf,
) -> crate::Result<Option<ScanData>> {
    let fnt_dir = content_dir.join("Fonts");

    if !fnt_dir.is_dir() {
        reporter.report_init(0);
        reporter.report_completed(Some("`Fonts` folder not found. Skipped.".into()));
        return Ok(None);
    }

    let fnt_ref = ref_dir.join("fonts.txt");
    let fnt_ref = Arc::new(FontRef::load(&fnt_ref, fnt_dir.clone())?);

    let task_size = WalkDir::new(&fnt_dir).into_iter().count();
    reporter.report_init(task_size);

    let asset_count = fnt_ref.data.len() as u16;
    let (sender, receiver) = mpsc::channel();

    WalkDir::new(fnt_dir)
        .into_iter()
        .par_bridge()
        .try_for_each_with((sender, fnt_ref), process_entry)?;

    let mut data = ScanData::new(asset_count);

    for result in receiver.iter() {
        match result {
            ScanResult::Valid => data.replaced += 1,
            ScanResult::Skipped => {}
            ScanResult::Invalid(i) => data.problems.push(i),
        }

        reporter.report_update(Update::Processed(1));
    }

    reporter.report_completed(None);
    Ok(Some(data))
}

fn process_entry(
    (sender, fnt_ref): &mut (Sender<ScanResult>, Arc<FontRef>),
    entry: walkdir::Result<DirEntry>,
) -> crate::Result<()> {
    let entry = entry?;

    if !entry.file_type().is_file() {
        sender.send(ScanResult::Skipped).unwrap();
        return Ok(());
    }

    let path = entry.into_path();

    if path.extension() != Some(OsStr::new("xnb")) {
        sender
            .send(InvalidAsset::new(path, "not an xnb file").into())
            .unwrap();

        return Ok(());
    }

    sender.send(fnt_ref.validate(&path)?).unwrap();
    Ok(())
}
