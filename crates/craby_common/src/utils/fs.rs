use std::{fs, path::PathBuf};

use log::debug;

pub fn collect_files(
    dir: &PathBuf,
    filter: &dyn Fn(&PathBuf) -> bool,
) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut files = Vec::new();

    if !dir.try_exists()? {
        debug!("Directory does not exist: {}", dir.display());
        return Ok(files);
    } else {
        debug!("Collecting files from: {}", dir.display());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if filter(&path) {
                files.push(path);
            }
        } else if path.is_dir() {
            files.extend(collect_files(&path, filter)?);
        }
    }

    Ok(files)
}
