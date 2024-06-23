use std::{fs, path::PathBuf, time::SystemTime};

/// Get the modification time of a file
pub fn get_modification_time(file_path: PathBuf) -> Option<SystemTime> {
    fs::metadata(file_path).map_or(None, |metadata| Some(metadata.modified().unwrap()))
}

/// Check if a file has been modified since a certain time
pub fn has_file_been_modified(file_path: PathBuf, last_checked_at: SystemTime) -> bool {
    get_modification_time(file_path).map_or(false, |modified_at| modified_at > last_checked_at)
}
