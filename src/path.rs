use std::{ffi::OsStr, fs, io::Error, path::PathBuf};



pub fn gimbal_files_in_path(app_path: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let read_dir = fs::read_dir(app_path)?;
    Ok(read_dir 
    .map(|res| res.ok())
    .filter(|dir_opt| dir_opt.is_some())
    .map(|dir_opt| dir_opt.unwrap().path())
    .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("gmd")))
    .collect::<Vec<PathBuf>>())
}