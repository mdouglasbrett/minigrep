use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // TODO: using clone would probably not be idiomatic in a larger program,
        // and may be revisited in a later exercise in the book.
        // I suspect _lifetimes_ 🙀
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path)?;
    println!("With text \n{contents}");
    // Idiomatic way of indicating the function is called for side effects only
    Ok(())
}
