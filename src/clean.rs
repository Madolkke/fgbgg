use std::path::PathBuf;

use crate::config::Config;



pub fn clean_dir(dir: &PathBuf){
    let file_paths = crate::preprocess::get_files_in_dir(dir);
    for file_path in file_paths{
        let _ = std::fs::remove_file(file_path);
    }
}

