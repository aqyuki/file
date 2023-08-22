use std::{env, path::Path};

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
    let path = Path::new(target);
    if !path.is_file() {
        println!("Could not find target file.");
        return;
    }

    println!("Args {:?}", args);
}
