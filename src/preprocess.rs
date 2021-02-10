use std::{path::PathBuf};


pub fn preprocess(config: &crate::config::Config){
    let p_config = config.preprocess.clone();
    if p_config.clone{
        unimplemented!()
    }else{
        println!("Reformatting source images.");
        let f_path = PathBuf::from(config.task.foreground.clone());
        let b_path = PathBuf::from(config.task.background.clone());
        if f_path.is_dir() && b_path.is_dir(){
            reformat_then_rename_all(&f_path, p_config.format_target.as_str());
            reformat_then_rename_all(&b_path, p_config.format_target.as_str());
        }else{
            println!("[Preprocess Error] Invalid source image path.");
        }
    }
}

pub fn get_files_in_dir(path: &PathBuf) -> Vec<PathBuf>{
    let mut result = Vec::new();
    for entry in std::fs::read_dir(path).unwrap(){
        let path = entry.unwrap().path();
        if path.is_file(){
            result.push(path);
        }
    }
    result
}

pub fn reformat_then_rename_all(path: &PathBuf, target_format: &str){
    // path is the parent dir of files to be processed
    if !check_supported_target_format(target_format){ return; }
    let file_paths = get_files_in_dir(&path);
    let mut counter: usize = 0;
    for file in file_paths{
        if check_supported_image_format(&file){
            while !reformat_then_rename(&file, target_format, counter) { counter += 1;}
            counter += 1;
        }
    }
}



pub fn reformat_then_rename(path: &PathBuf, target_format: &str, name: usize) -> bool{
    let new_file_name = format!("{}.{}", name, target_format);
    let image = image::open(&path).unwrap();
    if let Err(_) = image.save(path.with_file_name(new_file_name)){
        return false;
    }
    let _ = std::fs::remove_file(path);
    true
}

pub fn check_supported_image_format(path: &PathBuf) -> bool{
    // path must be a file
    let extension = path.extension().unwrap().to_str().unwrap();
    if !(extension == "jpg" || extension == "png" || extension == "jpeg" || extension == "webp" || extension == "gif"){
        println!("[Unsupported Format Error] Image format [.{}] is not supported.", extension);
    }
    true
}

pub fn check_reformat_source(path: &PathBuf) -> bool{
    if !path.is_file() && !check_supported_image_format(path){
        println!("[Reformat Error] Invalid image file path `{}`.", path.to_str().unwrap());
        return false;
    }
    true
}

pub fn check_supported_target_format(target_format: &str) -> bool{
    let tf = target_format.to_ascii_lowercase();
    if !(tf == "jpg" || tf == "jpeg" || tf == "png" || tf == "webp"){
        println!("[Unsupported Target Format Error] Target format [.{}] is not supported.", tf);
        return false;
    }
    true
}

