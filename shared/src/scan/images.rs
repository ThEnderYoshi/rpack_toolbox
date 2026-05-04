//! Defines the logic for scanning images.

use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::{
        Arc,
        mpsc::{self, Sender},
    },
};

use imagesize::ImageSize;
use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::{DirEntry, WalkDir};

use crate::{
    ref_files,
    reporter::{Reporter, Update},
    scan::{InvalidAsset, ScanData, ScanResult},
};

struct ImageRef {
    data: HashMap<PathBuf, ImageSize>,
    root: PathBuf,
}

impl ImageRef {
    fn validate(&self, path: &Path) -> crate::Result<ScanResult> {
        let ref_path = path.strip_prefix(&self.root)?.with_extension("");

        let Some(ref_size) = self.data.get(&ref_path) else {
            return Ok(InvalidAsset::new(ref_path, "invalid file name").into());
        };

        let size = imagesize::size(path)?;

        Ok(if size == *ref_size {
            ScanResult::Valid
        } else {
            InvalidAsset::new(
                ref_path,
                format!(
                    "wrong image size (expected {}, got {})",
                    ref_files::get_size_string(*ref_size),
                    ref_files::get_size_string(size),
                ),
            )
            .into()
        })
    }
}
// type ImageRef = HashMap<PathBuf, ImageSize>;

pub fn scan(
    mut reporter: impl Reporter,
    content_dir: PathBuf,
    ref_dir: PathBuf,
) -> crate::Result<Option<ScanData>> {
    let img_dir = content_dir.join("Images");

    if !img_dir.is_dir() {
        reporter.report_init(0);
        reporter.report_completed(Some("`Images` folder not found. Skipped.".into()));
        return Ok(None);
    }

    let img_ref = ref_dir.join("images.txt");
    let img_ref = Arc::new(load_image_ref(&img_ref, img_dir.clone())?);

    let task_size = WalkDir::new(&img_dir).into_iter().count();
    reporter.report_init(task_size);

    let asset_count = img_ref.data.len() as u16;
    let (sender, receiver) = mpsc::channel();

    WalkDir::new(img_dir)
        .into_iter()
        .par_bridge()
        .try_for_each_with((sender, img_ref), process_entry)?;

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
    (sender, img_ref): &mut (Sender<ScanResult>, Arc<ImageRef>),
    entry: walkdir::Result<DirEntry>,
) -> crate::Result<()> {
    let entry = entry?;

    if !entry.file_type().is_file() {
        sender.send(ScanResult::Skipped).unwrap();
        return Ok(());
    }

    let path = entry.into_path();

    if path.extension() != Some(OsStr::new("png")) {
        sender
            .send(
                InvalidAsset {
                    path,
                    msg: "not a png file".into(),
                }
                .into(),
            )
            .unwrap();

        return Ok(());
    }

    let result = img_ref.validate(&path)?;
    sender.send(result).unwrap();
    Ok(())
}

fn load_image_ref(path: &Path, root: PathBuf) -> crate::Result<ImageRef> {
    let raw = ref_files::read_tree_ref(path, ref_files::FORMAT_IMG)?;
    let mut data = HashMap::<PathBuf, _>::new();

    for (dir, names_sizes) in raw {
        for name_size in names_sizes {
            let (name, size) = name_size
                .rsplit_once(' ')
                .ok_or_else(|| crate::Error::bad_ref_file(path, "invalid image reference"))?;

            let size = ref_files::parse_size_string(size)
                .ok_or_else(|| crate::Error::bad_ref_file(path, "invalid image size"))?;

            if dir.is_empty() {
                data.insert(name.into(), size);
            } else {
                data.insert(format!("{dir}/{name}").into(), size);
            }
        }
    }

    Ok(ImageRef { data, root })
}
