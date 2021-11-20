use std::io::Error;
use std::path::Path;
use std::{fs::File, io::Write};

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_string_into_file(path: &Path, stringified_json: String) -> Result<(), Error> {
    if let Some(prefix) = path.parent() {
        std::fs::create_dir_all(prefix)?;
    }
    let mut log_file = File::create(path)?;
    log_file.write_all(stringified_json.as_bytes())?;
    log_file.sync_all()?;
    Ok(())
}
