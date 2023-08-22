use std::env;

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



    println!("Args {:?}", args);
}
