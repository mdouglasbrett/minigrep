use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text \n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // TODO: using clone would probably not be idiomatic in a larger program,
        // and may be revisited in a later exercise in the book.
        // I suspect _lifetimes_ 🙀
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
