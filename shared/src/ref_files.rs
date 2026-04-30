//! Defines members related to the reference files created by
//! [`generate`][crate::generate].

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, /*BufRead,*/ Write},
    path::Path,
};

use imagesize::ImageSize;

/// The format version of the image reference file.
pub const FORMAT_IMG: u8 = 0;

/// The format version of the translation reference file.
pub const FORMAT_TXT: u8 = 0;

/// The format version of the music reference file.
pub const FORMAT_MUS: u8 = 0;

/// The format version of the sound reference file.
pub const FORMAT_SND: u8 = 0;

/// The format version of the font reference file.
pub const FORMAT_FNT: u8 = 0;

/// Alias for the structure of a tree reference file.
pub type TreeRef = HashMap<String, HashSet<String>>;

/// Alias for the structure of a flat reference file.
pub type FlatRef = HashSet<String>;

/// Creates a [`TreeRef`] key-value pair from the provided `path` and
/// `root` dir.
///
/// Returns [`None`] if the arguments are invalid.
pub fn get_tree_key_value(root: &Path, path: &Path) -> Option<(String, String)> {
    let dir = path.parent()?.strip_prefix(root).ok()?;
    let dir = dir.to_string_lossy().to_string();
    let file = get_flat_value(path)?;
    Some((dir, file))
}

/// Creates a [`FlatRef`] value from the provided `path`.
///
/// Returns [`None`] if the argument is invalid.
pub fn get_flat_value(path: &Path) -> Option<String> {
    Some(path.file_name()?.to_string_lossy().to_string())
}

/// Writes a [`TreeRef`] to the file at the provided [`Path`].
pub fn write_tree_ref(path: &Path, format_version: u8, data: TreeRef) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "{format_version}")?;

    for (dir, names) in data {
        writeln!(file, "/{dir}")?;

        for name in names {
            debug_assert!(!name.starts_with('/'), "names must not start with `/`");
            writeln!(file, "{name}")?;
        }
    }

    Ok(())
}

/// Writes a [`FlatRef`] to the file at the provided [`Path`].
pub fn write_flat_ref(path: &Path, format_version: u8, data: FlatRef) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "{format_version}")?;

    for name in data {
        writeln!(file, "{name}")?;
    }

    Ok(())
}

// pub fn read_tree_ref(path: &Path, format_version: u8) -> crate::Result<TreeRef> {
//     let file = io::BufReader::new(File::open(path)?);
//     let mut lines = file.lines();

//     // Read format version
//     let actual: u8 = lines
//         .next()
//         .ok_or_else(|| crate::Error::no_format_version(path))??
//         .parse()
//         .map_err(|_| crate::Error::no_format_version(path))?;

//     if actual != format_version {
//         return Err(crate::Error::bad_format_version(
//             path,
//             format_version,
//             actual,
//         ));
//     }

//     // Read first dir
//     let Some(dir) = lines.next() else {
//         // File is empty. This shouldn't happen if the file was created by
//         // `generate`, but it's technically valid
//         return Ok(TreeRef::new());
//     };

//     let dir = dir?;

//     if !dir.starts_with('/') {
//         return Err(crate::Error::bad_ref_file(
//             path,
//             "second line of tree ref must be a directory",
//         ));
//     }

//     let mut tree = TreeRef::new();
//     tree.insert(dir.clone(), HashSet::new());
//     let mut current_set = tree.get_mut(&dir).unwrap(); // Never panics

//     // Read remaining lines
//     for line in lines {
//         let line = line?;

//         // Add name to current dir
//         if !line.starts_with('/') {
//             current_set.insert(line);
//             continue;
//         }

//         // Add dir
//         tree.insert(line.clone(), HashSet::new());
//         current_set = tree.get_mut(&line).unwrap(); // Never panics
//     }

//     Ok(tree)
// }

/// Converts an [`ImageSize`] into its [`String`] representation.
///
/// See [`parse_size_string`] for the string format.
pub fn get_size_string(size: ImageSize) -> String {
    format!("{}x{}", size.width, size.height)
}

// /// Parses the provided string into an [`ImageSize`], or returns [`None`] if it
// /// doesn't follow the format.
// ///
// /// `raw` is expected to be in the format `<W>x<H>`, where `<W>` and `<H>` are
// /// valid [`usize`] strings.
// ///
// /// See also [`get_size_string`].
// pub fn parse_size_string(raw: &str) -> Option<ImageSize> {
//     let (width, height) = raw.split_once('x')?;
//     let width: usize = width.parse().ok()?;
//     let height: usize = height.parse().ok()?;
//     Some(ImageSize { width, height })
// }
