use chrono::{
    prelude::{DateTime, Datelike, Utc},
    Timelike,
};

use colored::*;
use std::{env, os::windows::prelude::MetadataExt, path::PathBuf};

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("arguments error");
        return;
    }

    let target = match args.get(1) {
        Some(elem) => elem,
        None => {
            println!("Failure get filepath");
            return;
        }
    };

    let path = PathBuf::from(&target);
    show_file_info(&path)
}

fn show_file_info(path: &PathBuf) {
    if !path.is_file() {
        println!("Could not find target file.");
        return;
    }

    let info = match std::fs::metadata(&path) {
        Ok(file) => file,
        Err(why) => {
            println!("Failure open file {}", why);
            std::process::exit(-1);
        }
    };

    let parent_directory = match path.parent() {
        Some(parent) => parent.to_str().unwrap().normal(),
        None => "No information".red(),
    };

    let file_name = match path.file_name() {
        Some(name) => name.to_str().unwrap().normal(),
        None => "Failure get file name".red(),
    };

    let created_utc: DateTime<Utc> = info.created().unwrap().into();
    let created_at = format_utc_to_string(&created_utc).green();

    let file_size = info.file_size().to_string().blue();

    println!("Parent Directory : {}", parent_directory);
    println!("File Name : {}", file_name);
    println!("Created At : {}", created_at);
    println!("File Size : {}", file_size);
}

fn format_utc_to_string(utc_time: &DateTime<Utc>) -> String {
    format!(
        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        utc_time.year(),
        utc_time.month(),
        utc_time.day(),
        utc_time.hour(),
        utc_time.minute(),
        utc_time.second(),
    )
}
