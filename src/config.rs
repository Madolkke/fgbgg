use serde::Deserialize;
use std::{io::Read, str::FromStr};


pub fn load_local_config() -> Option<Config>{
    let mut path = std::env::current_exe().unwrap();      
    path.pop();
    path = path.join("task.toml");
    if !path.exists(){
        path = std::env::current_dir().unwrap();
        path.pop();
        path = path.join("task.toml");
    }
    if !path.exists() {
        println!("Local config not found in `{}`.", path.to_str().unwrap());
        return None;
    }
    if let Ok(mut file) = std::fs::File::open(path.clone()){
        let mut buf = String::new();
        let _ = file.read_to_string(&mut buf);
        let config: Config = toml::from_str(&buf).unwrap();
        println!("Load configuration from `{}`.", path.to_str().unwrap());
        return Some(config);
    }
    println!("Local config not found in `{}`.", path.to_str().unwrap());
    None
}

pub fn load_config(path: &str) -> Option<Config>{
    let path = std::path::PathBuf::from_str(path).unwrap();
    if path.exists(){
        if let Ok(mut file) = std::fs::File::open(path.clone()){
            let mut buf = String::new();
            let _  = file.read_to_string(&mut buf);
            let config: Config = toml::from_str(&buf).unwrap();
            println!("Load configuration from `{}`.", path.to_str().unwrap());
            Some(config);
        }
        return None;
    }
    println!("Config not found.");
    None
}

#[derive(Deserialize, Clone)]
pub struct Config{
    pub task: Task,
    pub generate: Generate,
    pub preprocess: Preprocess,
    pub statistic: Option<Statistic>,
    pub tag: Option<Tag>
}

#[derive(Deserialize, Clone)]
pub struct Task{
    pub name: String,
    pub foreground: String,
    pub background: String,
    pub result: String,
    pub tag: bool
}

#[derive(Deserialize, Clone)]
pub struct Generate{
    pub foreground_position: String,
    pub threshold_max: f32,
    pub threshold_min: f32,
    pub mode: String,
    pub counter: usize,
    pub save_alpha: bool,
    pub save_jpg: bool
}

#[derive(Deserialize, Clone)]
pub struct Preprocess{
    pub format_target: String,
    pub clone: bool,
}

#[derive(Deserialize, Clone)]
pub struct Statistic{
    pub source: String,
    pub target: String,
    pub base: String
}

#[derive(Deserialize, Clone)]
pub struct Tag{

}