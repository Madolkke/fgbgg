mod config;
mod preprocess;
mod generate;
mod statistic;
mod algorithm;
mod clean;

use std::path::PathBuf;
use clap::{Arg, App, SubCommand};



fn main() {

    let sc_preprocess = 
    SubCommand::with_name("preprocess")
        .about("Preprocess source images.")
        .version("0.1")
        .arg(
            Arg::with_name("path")
                .long("path")
                .short("p")
                .required(false)
                .help("Path of task config file.")
        );

    let sc_generate = 
    SubCommand::with_name("generate")
        .about("Generate foreground-background images.")
        .version("0.1")
        .arg(
            Arg::with_name("path")
            .long("path")
            .short("p")
            .required(false)
            .help("Path of task config file.")
        );

    let sc_statistic = 
    SubCommand::with_name("statistic")
        .about("Comupute statistic data for alpha channels.")
        .version("0.1")
        .arg(
            Arg::with_name("path")
            .long("path")
            .short("p")
            .required(false)
            .help("Path of task config file.")
        );

    let sc_clean = 
    SubCommand::with_name("clean")
        .about("Clean workspace.")
        .version("0.1")
        .arg(
            Arg::with_name("result")
            .long("result")
            .short("r")
            .help("Clean files in ./result/3-channel & ./result/origin folder.")
        )
        .arg(
            Arg::with_name("path")
            .long("path")
            .short("p")
            .required(false)
            .help("Path of task config file.")
        )
        .arg(
            Arg::with_name("source")
            .long("source")
            .short("s")
            .help("Clean files in ./foreground & ./background folder")
        );

    let m = 
    App::new("Foreground Background Generator")
        .version("0.0.1")
        .author("Madolkke <chaoxvailor@gmail.com>")
        .subcommand(sc_preprocess)
        .subcommand(sc_generate)
        .subcommand(sc_statistic)
        .subcommand(sc_clean)
        .get_matches();

    match m.subcommand(){
        ("clean", Some(args)) => {
            let config: Option<config::Config>;
            if args.is_present("path") { config = config::load_config(args.value_of("path").unwrap()); }
            else{ config = config::load_local_config(); }
            if let None = config{ return; }
            let config = config.unwrap();
            let f_path = PathBuf::from(config.task.foreground);
            let b_path = PathBuf::from(config.task.background);
            let r_path = PathBuf::from(config.task.result);

            if args.is_present("source"){
                crate::clean::clean_dir(&f_path);
                crate::clean::clean_dir(&b_path);
            }
            if args.is_present("result"){
                crate::clean::clean_dir(&r_path.join("3-channel"));
                crate::clean::clean_dir(&r_path.join("origin"));
                crate::clean::clean_dir(&r_path.join("alpha"));
            }
        }
        ("statistic", Some(args)) => {
            let config: Option<config::Config>;
            if args.is_present("path") { config = config::load_config(args.value_of("path").unwrap()); }
            else{ config = config::load_local_config(); }
            if let None = config{ return; }
            let config = config.unwrap();
            statistic::statistic(&config);
        }
        ("generate", Some(args)) => {
            let config: Option<config::Config>;
            if args.is_present("path") { config = config::load_config(args.value_of("path").unwrap()); }
            else{ config = config::load_local_config(); }
            if let None = config{ return; }
            let config = config.unwrap();
            println!("Ready to generate images.");
            generate::generate(&config);
            println!("Generation finished.");
        }
        ("preprocess", Some(args)) => {
            let config: Option<config::Config>;
            if args.is_present("path") { config = config::load_config(args.value_of("path").unwrap()); }
            else{ config = config::load_local_config(); }
            if let None = config{ return; }
            let config = config.unwrap();
            preprocess::preprocess(&config);
            println!("Preprocess finished.");

        }
        _ => { println!("Invalid Command."); }
    }
}
