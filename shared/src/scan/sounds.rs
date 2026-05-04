//! Defines the logic for scanning sounds.

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

struct SoundRef {
    data: HashSet<PathBuf>,
    root: PathBuf,
}

impl SoundRef {
    fn load(path: &Path, root: PathBuf) -> crate::Result<Self> {
        let raw = ref_files::read_tree_ref(path, ref_files::FORMAT_SND)?;
        let mut data = HashSet::new();

        for (dir, names) in raw {
            for name in names {
                if dir.is_empty() {
                    data.insert(name.into());
                } else {
                    data.insert(format!("{dir}/{name}").into());
                }
            }
        }

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
    let snd_dir = content_dir.join("Sounds");

    if !snd_dir.is_dir() {
        reporter.report_init(0);
        reporter.report_completed(Some("`Sounds` folder not found. Skipped.".into()));
        return Ok(None);
    }

    let snd_ref = ref_dir.join("sounds.txt");
    let snd_ref = Arc::new(SoundRef::load(&snd_ref, snd_dir.clone())?);

    let task_size = WalkDir::new(&snd_dir).into_iter().count();
    reporter.report_init(task_size);

    let asset_count = snd_ref.data.len() as u16;
    let (sender, receiver) = mpsc::channel();

    WalkDir::new(snd_dir)
        .into_iter()
        .par_bridge()
        .try_for_each_with((sender, snd_ref), process_entry)?;

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
    (sender, snd_ref): &mut (Sender<ScanResult>, Arc<SoundRef>),
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

    sender.send(snd_ref.validate(&path)?).unwrap();
    Ok(())
}
