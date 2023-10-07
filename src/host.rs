use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// List all files in a single directory. This function can recurse into sub
/// directores.
///
/// This function will recurse into all sub-directories as well.
///
/// # Example
///
/// ```no_run
/// use crate::host;
///
/// let path = "~/directory";
/// let files = host::list_files(&path, 1).unwrap();
/// ```
pub fn list_files(source: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut file_entries = vec![];

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;

        if filetype.is_dir() {
            file_entries.append(&mut list_files(entry.path())?);
        } else if filetype.is_file() {
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
