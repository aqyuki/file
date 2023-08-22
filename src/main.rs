use chrono::{
    prelude::{DateTime, Datelike, Utc},
    Timelike,
};

use std::{env, os::windows::prelude::MetadataExt, path::Path};

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

    let path = Path::new(&target);
    show_file_info(&path)
}

fn show_file_info(path: &Path) {
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

    let created_utc: DateTime<Utc> = info.created().unwrap().into();
    let created_at = format_utc_to_string(&created_utc);

    let file_size = info.file_size();

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
