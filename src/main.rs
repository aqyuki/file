extern crate chrono;
use chrono::{
    prelude::{DateTime, Datelike, Utc},
    Timelike,
};

use std::{env, fs::File, path::Path};

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

    // Check file exist
    let path = Path::new(&target);
    if !path.is_file() {
        println!("Could not find target file.");
        return;
    }

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            println!("Failure open file {}", why);
            std::process::exit(-1);
        }
    };

    let info = file.metadata().unwrap();

    let created_utc: DateTime<Utc> = info.created().unwrap().into();
    let created_at = format_utc_to_string(&created_utc);

    println!("Created At : {}", created_at)
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
