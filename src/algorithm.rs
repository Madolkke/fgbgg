use opencv::core::{Mat, MatTrait, Vector};






pub fn remove_alpha_channel(img: &Mat) -> Mat{
    if img.channels().unwrap() != 4{
        println!("[Invalid Image Error] Not a 4-channels image. Channel(s): `{}`", img.channels().unwrap());
        return Mat::default().unwrap();
    }
    let mut channels: Vector<Mat> = Vector::new();
    let _ = opencv::core::split(&img.clone(), &mut channels);
    let _ = channels.remove(channels.len() - 1);
    let mut result = Mat::default().unwrap();
    let _ = opencv::core::merge(&channels, &mut result);
    result
}


#[cfg(test)]
mod test{
    use super::remove_alpha_channel;

    #[test]
    pub fn msbwa_2014_test(){
        let p = r"D:\\Workspace\\fgbgg\\sample\\result\\b1f14p8.png";
        let img = opencv::imgcodecs::imread(p, -1).unwrap();
        let p = remove_alpha_channel(&img);
        let _ = opencv::highgui::imshow("test", &p);
        let _ = opencv::highgui::wait_key(0);
    }

}