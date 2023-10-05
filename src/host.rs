use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// List all files in a single directory. This function can recurse into sub
/// directores which is limited by [`depth`]. A [`depth`] of `0` will only list
/// files in [`target`].
///
/// This function only returns files of the given [`depth`], e.g. using a
/// [`depth`] of 1 and given the [`target`] `~/my-path`, the function would
/// return `~/my-path/train/0.jpg`, `~/my-path/train/1.jpg` and
/// `~/my-path/test/0.jpg` but not `~/my-path/a.jpg`.
///
/// # Example
///
/// ```no_run
/// use crate::host;
///
/// let path = "~/directory";
/// let files = host::list_files(&path, 1).unwrap();
/// ```
pub fn list_files(source: impl AsRef<Path>, depth: u8) -> Result<Vec<PathBuf>> {
    let mut file_entries = vec![];

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;

        if filetype.is_dir() && depth > 0 {
            file_entries.append(&mut list_files(entry.path(), depth - 1)?);
        } else if filetype.is_file() && depth == 0 {
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "eml" {
                    file_entries.push(path);
                }
            }
        }
    }

    Ok(file_entries)
}
