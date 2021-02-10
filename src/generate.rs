use std::path::PathBuf;
use opencv::{core::{Mat, MatTrait, MatTraitManual, Scalar, Size, Vector, merge, split}, highgui::{imshow, wait_key}, imgcodecs::{IMWRITE_PNG_COMPRESSION, imread, imwrite}};
use rand::prelude::SliceRandom;
use crate::config::Config;
use crate::preprocess::get_files_in_dir;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ROIPosition{
    Left, Right, Top, Bottom,
    TopLeft, TopRight, BottomLeft, BottomRight,
    Center
}

pub fn generate(config: &Config){
    let task = config.task.clone();
    let config = config.generate.clone();
    if !check_threshold(config.threshold_max, config.threshold_min){ return; }
    match config.mode.as_str(){
        "all" => {
            let f_paths = get_files_in_dir(&PathBuf::from(task.foreground));
            let b_paths = get_files_in_dir(&PathBuf::from(task.background));
            let r_path = PathBuf::from(task.result);
            if config.foreground_position == "random"{
                for f in &f_paths{
                    for b in &b_paths{
                        mix_with_fixed_roi(
                            f, b, &r_path, 
                            config.threshold_max, config.threshold_min, 
                            config.save_alpha, config.save_jpg, 
                            &random_roi_position()
                        );
                    }
                }
            }else if let Some(position) = check_position(config.foreground_position){
                for f in &f_paths{
                    for b in &b_paths{
                        mix_with_fixed_roi(
                            f, b, &r_path, 
                            config.threshold_max, config.threshold_min, 
                            config.save_alpha, config.save_jpg, 
                            &position
                        );
                    }
                }
            }
            return;
        }
        "loop" => {
            let mut counter = config.counter;
            let f_paths = get_files_in_dir(&PathBuf::from(task.foreground));
            let b_paths = get_files_in_dir(&PathBuf::from(task.background));
            let r_path = PathBuf::from(task.result);
            let mut f_counter: usize = 0;
            let mut b_counter: usize = 0;
            if config.foreground_position == "random"{
                println!("kkk");
                while counter > 0{
                    mix_with_fixed_roi(
                        &f_paths[f_counter], &b_paths[b_counter], &r_path, 
                        config.threshold_max, config.threshold_min, 
                        config.save_alpha, config.save_jpg, 
                        &random_roi_position()
                    );
                    if f_counter == f_paths.len() - 1{
                        f_counter = 0;
                    }else { f_counter += 1; }
                    if b_counter == b_paths.len() - 1{
                        b_counter = 0;
                    }else { b_counter += 1; }
                    counter -= 1;
                }
            }else if let Some(position) = check_position(config.foreground_position){
                while counter > 0{
                    mix_with_fixed_roi(
                        &f_paths[f_counter], &b_paths[b_counter], &r_path, 
                        config.threshold_max, config.threshold_min, 
                        config.save_alpha, config.save_jpg, 
                        &position
                    );
                    if f_counter == f_paths.len() - 1{
                        f_counter = 0;
                    }else { f_counter += 1; }
                    if b_counter == b_paths.len() - 1{
                        b_counter = 0;
                    }else { b_counter += 1; }
                    counter -= 1;
                }
            }
            return;
        }
        "random" => {
            let mut counter = config.counter;
            let mut f_paths = get_files_in_dir(&PathBuf::from(task.foreground));
            let mut b_paths = get_files_in_dir(&PathBuf::from(task.background));
            let r_path = PathBuf::from(task.result);
            if config.foreground_position == "random"{
                while counter > 0{
                    let mut rng = rand::thread_rng();
                    f_paths.shuffle(&mut rng);
                    b_paths.shuffle(&mut rng);
                    mix_with_fixed_roi(
                        &f_paths[0], &b_paths[0], &r_path, 
                        config.threshold_max, config.threshold_min,
                        config.save_alpha, config.save_jpg,
                        &random_roi_position()
                    );
                    counter -= 1;
                }
            }else if let Some(position) = check_position(config.foreground_position){
                while counter > 0{
                    let mut rng = rand::thread_rng();
                    f_paths.shuffle(&mut rng);
                    b_paths.shuffle(&mut rng);
                    mix_with_fixed_roi(
                        &f_paths[0], &b_paths[0], &r_path, 
                        config.threshold_max, config.threshold_min, 
                        config.save_alpha, config.save_jpg,
                        &position
                    );
                    counter -= 1;
                }
            }else{ return; }

        }
        _ => {
            println!("[Generate Mode Error] Generate mode `{}` not supported", config.mode);
            return;
        }
    }



}


pub fn mix_with_fixed_roi(
    f_path: &PathBuf, b_path: &PathBuf, r_path: &PathBuf, 
    threshold_max: f32, threshold_min: f32, 
    save_alpha: bool, save_jpg: bool,
    position: &ROIPosition
)
{   
    dbg!(position);
    // f_path/b_path should be: Existed -> File -> Image
    // threshold should be checked
    let mut f_img = imread(f_path.to_str().unwrap(), opencv::imgcodecs::IMREAD_UNCHANGED).unwrap();
    let mut b_img = imread(b_path.to_str().unwrap(), opencv::imgcodecs::IMREAD_UNCHANGED).unwrap();

    apply_alpha(&mut f_img);

    // f_img / b_img should have 4 channels
    let mut b_img = assert_4_channels(b_img);

    let f_size = f_img.size().unwrap();
    let b_size = b_img.size().unwrap();
    let size_rate = (f_size.height * f_size.width) as f32 / (b_size.height * b_size.width) as f32;
    let resize_target = (threshold_max + threshold_min) / 2.0;
    if size_rate < threshold_min{
        let k = (size_rate / resize_target).sqrt();
        let p = &mut b_img as *mut Mat;
        let _ = unsafe{ opencv::imgproc::resize(&b_img, &mut *p, Size::default(), k as f64, k as f64, opencv::imgproc::INTER_CUBIC) };
    }else if size_rate > threshold_max{
        let k = (resize_target / size_rate).sqrt();
        let p = &mut f_img as *mut Mat;
        let _ = unsafe{ opencv::imgproc::resize(&f_img, &mut *p, Size::default(), k as f64, k as f64, opencv::imgproc::INTER_NEAREST) };
    }
    let f_size = f_img.size().unwrap();
    let b_size = b_img.size().unwrap();
    let (f_width, f_height) = (f_size.width, f_size.height);
    let (b_width, b_height) = (b_size.width, b_size.height);
    use opencv::core::Rect;
    let roi: opencv::core::Rect_<i32> = match position{
        ROIPosition::Left => { Rect::new(0, (b_height - f_height)/2, f_width, f_height) }
        ROIPosition::Right => { Rect::new(b_width - f_width, (b_height - f_height)/2, f_width, f_height) }
        ROIPosition::Top => { Rect::new((b_width - f_width)/2, 0, f_width, f_height) }
        ROIPosition::Bottom => { Rect::new((b_width - f_width)/2, b_height - f_height, f_width, f_height) }
        ROIPosition::TopLeft => { Rect::new(0, 0, f_width, f_height) }
        ROIPosition::TopRight => { Rect::new(b_width - f_width, 0, f_width, f_height) }
        ROIPosition::BottomLeft => { Rect::new(0, b_height - f_height, f_width, f_height) }
        ROIPosition::BottomRight => { Rect::new(b_width - f_width, b_height - f_height, f_width, f_height) }
        ROIPosition::Center => { Rect::new((b_width - f_width)/2, (b_height - f_height)/2, f_width, f_height) }
    };
    let k = Mat::roi(&b_img, roi);
    if let Err(_) = k{ return; }
    let mut k = k.unwrap();

    let _ = &f_img.copy_to_masked(&mut k, &get_alpha(&f_img));
    let name = format!("b{}f{}p{:?}", b_path.file_stem().unwrap().to_str().unwrap(), f_path.file_stem().unwrap().to_str().unwrap(), position_to_int(&position));
    let compression_params: Vector<i32> = Vector::from(vec![
        IMWRITE_PNG_COMPRESSION, 9,
    ]);
    
    save_config(&b_img, &b_img, &r_path, save_alpha, save_jpg, &name);
    let _ = imwrite(r_path.join("origin").join(name).with_extension("png").to_str().unwrap(), &b_img, &compression_params).unwrap();
}


fn check_threshold(max: f32, min: f32) -> bool{
    if min > max{
        println!("[Invalid Threshold Error] Min:{}  Max:{}", min, max);
        return false;
    }
    if min < 0.0000001{
        println!("[Threshold Underflow Error] Min:{}  Max:{}", min, max);
        return false;
    }
    if max > 0.9999990{
        println!("[Threshold Overflow Error] Min:{}  Max:{}", min, max);
        return false;
    }
    true
}

fn check_position(position: String) -> Option<ROIPosition>{
    use ROIPosition::*;
    match position.as_str(){
        "left" => { Some(Left) }
        "right" => { Some(Right) }
        "top" => { Some(Top) }
        "bottom" => { Some(Bottom) }
        "top-left" => { Some(TopLeft) }
        "top-right" => { Some(TopRight) }
        "bottom-left" => { Some(BottomLeft) }
        "bottom-right" => { Some(BottomRight) }
        "center" => { Some(Center) }
        _ => {
            println!("[Invalid Position Error] Position `{}` Not Supported", position);
            None
        }
    }
}

fn show(m: &Mat){
    let _ = imshow("test", m);
    let _ = wait_key(0);
}

pub fn assert_4_channels(mut m: Mat) -> Mat{
    if m.channels().unwrap() == 4{
        return m;
    }
    if m.channels().unwrap() != 3{
        panic!("Unsupported channels.");
    }
    let alpha: Mat = Mat::new_size_with_default(Size::new(m.cols(), m.rows()), opencv::core::CV_8UC1, Scalar::all(0.0)).unwrap();
    //let alpha: Mat = Mat::zeros(m.rows(), m.cols(), opencv::core::CV_8UC1).unwrap().to_mat().unwrap();
    let mut channels: Vector<Mat> = Vector::new();
    let _ = split(&m, &mut channels);
    let _ = channels.insert(3, alpha);
    let _ = merge(&channels, &mut m);
    return m;
}

pub fn apply_alpha(m: &mut Mat){
    let mut channels: Vector<Mat> = Vector::new();
    let _ = opencv::core::split(m, &mut channels);
    let mut vec = channels.to_vec();
    let mut r1 = Mat::default().unwrap();
    let mut r2 = Mat::default().unwrap();
    let mut r3 = Mat::default().unwrap();
    let _ = opencv::core::bitwise_and(&vec[0], &vec[3], &mut r1, &vec[3]);
    let _ = opencv::core::bitwise_and(&vec[1], &vec[3], &mut r2, &vec[3]);
    let _ = opencv::core::bitwise_and(&vec[2], &vec[3], &mut r3, &vec[3]);
    let mut r4 = vec.pop().unwrap();
    let p = &mut r4 as *mut Mat;
    unsafe{ let _ = opencv::imgproc::threshold(&r4, &mut *p, 100.0, 256.0, opencv::imgproc::THRESH_BINARY); }
    let mv: Vector<Mat> = opencv::core::Vector::from(vec![r1, r2, r3, r4]);
    let _ = opencv::core::merge(&mv, m);
}

pub fn get_alpha(m: &Mat) -> Mat{
    let mut channels: Vector<Mat> = Vector::new();
    let _ = opencv::core::split(&m.clone(), &mut channels);
    let mut vec = channels.to_vec();
    vec.pop().unwrap()
}

pub fn show_channels(m: &Mat){
    let mut m = m.clone();
    let mut channels: Vector<Mat> = opencv::core::Vector::new();
    let _ = opencv::core::split(&mut m, &mut channels);
    let channels = channels.to_vec();
    let _ = imshow("channel0", &channels[0]);
    let _ = wait_key(0);
    let _ = imshow("channel1", &channels[1]);
    let _ = wait_key(0);
    let _ = imshow("channel2", &channels[2]);
    let _ = wait_key(0);
    let _ = imshow("channel3", &channels[3]);
    let _ = wait_key(0);
    let _ = imshow("channel4", &channels[4]);
    let _ = wait_key(0);

}

pub fn random_roi_position() -> ROIPosition{
    use ROIPosition::*;
    let mut rng = rand::thread_rng();
    let mut nums: Vec<u8> = (0..8).collect();
    nums.shuffle(&mut rng);
    match nums[0]{
        0 => { Center }
        1 => { Left }
        2 => { Right }
        3 => { Top }
        4 => { Bottom }
        5 => { TopLeft }
        6 => { TopRight }
        7 => { BottomLeft }
        8 => { BottomRight }
        _ => { panic!("RNG FAILED")}
    }
}

pub fn position_to_int(p: &ROIPosition) -> u8{
    use ROIPosition::*;
    match p{
        Center => 0,
        Left => 1,
        Right => 2,
        Top => 3,
        Bottom => 4,
        TopLeft => 5,
        TopRight => 6,
        BottomLeft => 7,
        BottomRight => 8
    }
}

pub fn int_to_position(n: u8) -> ROIPosition{
    use ROIPosition::*;
    match n{
        1 => Left,
        2 => Right,
        3 => Top,
        4 => Bottom,
        5 => TopLeft,
        6 => TopRight,
        7 => BottomLeft,
        8 => BottomRight,
        _ => Center
    }
}

pub fn save_config(origin: &Mat, f_img: &Mat, r_path: &PathBuf, save_alpha: bool, save_jpg: bool, name: &String){
    assert_eq!(origin.channels().unwrap(), 4);
    let mut channels: Vector<Mat> = Vector::new();
    let _ = opencv::core::split(&origin.clone(), &mut channels);
    let mut vec = channels.to_vec();
    let alpha = get_alpha(&f_img);
    if save_alpha{
        let _ = imwrite(r_path.join("alpha").join(name).with_extension("jpg").to_str().unwrap(), &alpha, &Vector::new());
    }
    if save_jpg{
        let mut c3 = Mat::default().unwrap();
        let s: Vector<Mat> = Vector::from(vec);
        let _ = opencv::core::merge(&s, &mut c3);
        let _ = imwrite(r_path.join("3-channel").join(name).with_extension("jpg").to_str().unwrap(), &c3, &Vector::new());
    }
}

#[cfg(test)]
mod test{

    use opencv::{core::{Mat, MatTrait, MatTraitManual, Scalar, Size, Vector, bitwise_and, merge, split}, highgui::{imshow, wait_key}, imgcodecs::{IMWRITE_PNG_COMPRESSION, imread, imwrite}};
    use crate::config::Config;

    use super::{random_roi_position, show, show_channels};

    #[test]
    pub fn test(){
        let f_path = std::path::PathBuf::from(r"D:\Workspace\fgbgg\sample\foreground\0.png");
        let b_path = std::path::PathBuf::from(r"D:\Workspace\fgbgg\sample\background\0.png");
        let r_path = std::path::PathBuf::from(r"D:\Workspace\fgbgg\sample\result");
        super::mix_with_fixed_roi(&f_path, &b_path, &r_path, 0.3, 0.2, false, false, &super::ROIPosition::Center);
    }

    #[test]
    pub fn add_alpha_channel(){
        let b_path = std::path::PathBuf::from(r"D:\Workspace\fgbgg\sample\background\0.png");
        let b_img = opencv::imgcodecs::imread(b_path.to_str().unwrap(), opencv::imgcodecs::IMREAD_UNCHANGED).unwrap();
        let p = super::assert_4_channels(b_img);
        println!("{:?}", p.channels());
        show(&p);
    }

    #[test]
    pub fn check_generated_png(){
        use opencv::core::{Vector, Mat};
        let r_path = std::path::PathBuf::from(r"D:\Workspace\fgbgg\sample\foreground\0.png");
        let mut png = opencv::imgcodecs::imread(r_path.to_str().unwrap(), -1).unwrap();
        
        let mut channels: Vector<Mat> = Vector::new();
        let _ = opencv::core::split(&mut png, &mut channels);
        let mut vec = channels.to_vec();
        let p0 = &mut vec[0] as *mut Mat;
        unsafe{ let _ = opencv::core::bitwise_and(&vec[0], &vec[3], &mut *p0, &vec[3]); }
        unsafe{show(&*p0);}
        show(&vec[1]);
        show(&vec[2]);
        show(&vec[3]);
    }

    #[test]
    pub fn alpha_apply_check(){
        let r_path = std::path::PathBuf::from(r"D:\Workspace\fgbgg\sample\foreground\0.png");
        let mut png = opencv::imgcodecs::imread(r_path.to_str().unwrap(), -1).unwrap();
        super::apply_alpha(&mut png);
        show_channels(&png);
    }

    #[test]
    pub fn roi_position_rand_gen_test(){
        for _ in (0..20).collect::<Vec<usize>>(){
            println!("{:?}", random_roi_position());
        }
    }
}