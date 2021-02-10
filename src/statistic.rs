use crate::{config::Config, generate::ROIPosition, preprocess::get_files_in_dir};
use opencv::{core::{Mat, MatTrait, Scalar, Size, bitwise_not}, prelude::MatTraitManual};
use std::path::PathBuf;
use std::collections::HashMap;

pub fn statistic(config: &Config){
    let s_config = config.statistic.clone().unwrap();
    let source_paths = get_files_in_dir(&PathBuf::from(s_config.source));
    let target_paths = get_files_in_dir(&PathBuf::from(s_config.target));
    let base = s_config.base;
    let mut target = HashMap::new();
    let mut result = HashMap::new();

    for target_path in target_paths{
        if let Some(t) = parse_file_name(&target_path.file_stem().unwrap().to_str().unwrap().to_string()){
            target.insert(t, target_path);
        }
    }
    for source_path in source_paths{
        let stem = &source_path.file_stem().unwrap().to_str().unwrap().to_string();
        if let Some(s) = parse_file_name(stem){
            if let Some(t) = target.get(&s){
                let miou = check_and_miou(&source_path, t);
                result.insert(s, (stem.clone(), miou));
            }else{
                println!("[Target Image Not Found Error] {}", &source_path.file_stem().unwrap().to_str().unwrap().to_string());
            }
        }
    }
    match base.as_str(){
        "background" => {

        }
        "foreground" => {

        }
        "position" => {

        }
        "default" => {
            for (k, v) in result{
                println!("[{:03}-{:03}-{:?}] ({:?})", k.0, k.1, k.2, v.1);
            }
        }
        _ => {
            println!("[Base Not Supported] {}", base);
            return;
        }
    }

}

pub fn check_and_miou(source: &PathBuf, target: &PathBuf) -> f64{
    let mut source = opencv::imgcodecs::imread(source.to_str().unwrap(), -1).unwrap();
    let mut target = opencv::imgcodecs::imread(target.to_str().unwrap(), -1).unwrap();
    if source.channels().unwrap() == 4{
        source = crate::generate::get_alpha(&source);
    }
    if target.channels().unwrap() == 4{
        target = crate::generate::get_alpha(&target);
    }
    if target.channels().unwrap() == 1 && source.channels().unwrap() == 1{
        return miou(&source, &target);
    }else{ panic!("Channel(s) Error!") }
}

pub fn miou(alpha1: &Mat, alpha2: &Mat) -> f64{
    if alpha1.channels().unwrap() != 1 || alpha2.channels().unwrap() != 1{
        println!("[Invalid Image Error] Too much channels: `{}`  `{}`", alpha1.channels().unwrap(), alpha2.channels().unwrap());
        return 0.0;
    }
    if alpha1.rows() != alpha2.rows() || alpha1.cols() != alpha2.cols(){
        println!("[Invalid Image Error] Image size not same: `{}*{}`  `{}*{}`.", alpha1.rows(), alpha1.cols(), alpha2.rows(), alpha2.cols());
        return 0.0;
    }
    let mut intersection = Mat::default().unwrap();
    let false_mask: Mat = Mat::new_size_with_default(Size::new(alpha1.cols(), alpha1.rows()), opencv::core::CV_8UC1, Scalar::all(255.0)).unwrap();
    let _ = opencv::core::bitwise_and(&alpha1, &alpha2, &mut intersection, &false_mask);
    let i_count = count_pixels(&intersection);
    i_count as f64 / (count_pixels(alpha1) + count_pixels(alpha2) - i_count) as f64
}

pub fn parse_file_name(file_name: &String) -> Option<(usize, usize, ROIPosition)>{
    use std::char;
    let s: Vec<&str> = file_name.split(char::is_alphabetic).collect();
    let mut s= s.iter();
    let _ = s.next();
    if let Some(b) = s.next(){

        let b = str::parse::<usize>(b).unwrap();
        if let Some(f) = s.next(){
            let f = str::parse::<usize>(f).unwrap();
            if let Some(p) = s.next(){
                let p = crate::generate::int_to_position(str::parse::<u8>(p).unwrap());
                return Some((b, f, p));
            }
        }
    }
    None
}


pub fn count_pixels(alpha: &Mat) -> usize{

    let mut row_counter = alpha.rows() - 1;
    let mut col_counter = alpha.cols() - 1;
    let mut result = 0;
    while row_counter > 0{
        while col_counter > 0{
            let p = alpha.at_2d::<u8>(row_counter, col_counter).unwrap();
            if p > &127{ result += 1; }
            col_counter -= 1;
        }
        row_counter -= 1;
    }
    result
}

#[cfg(test)]
mod test{
    use std::path::PathBuf;

    use opencv::{core::{Mat, MatTrait, Scalar, Size}, imgcodecs::imread, prelude::MatTraitManual};
    use crate::generate::get_alpha;
    use super::{check_and_miou, count_pixels, miou};

    #[test]
    pub fn miou_test(){
        

        let source = PathBuf::from(r"D:\\Workspace\\fgbgg\\sample\\result\\alpha\\b5f9p0.jpg");
        let target = PathBuf::from(r"D:\\Workspace\\animal-matting\\samples\\result_alpha\\b5f9p0.png");

        let img = imread(target.to_str().unwrap(), -1).unwrap();
        dbg!(count_pixels(&img));
        //let p = check_and_miou(&source, &target);
        //dbg!(p);

    }
}


