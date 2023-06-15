use std::{env, fs, process, error::Error};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // We only care about the Err here, so unwrap_or_else is uneeded
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path)?;
    println!("With text \n{contents}");
    // Idiomatic way of indicating the function is called for side effects only
    Ok(())
}
