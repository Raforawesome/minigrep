use std::env;
use std::fs;
use std::error::Error;

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Fatal error: {}", err);
        std::process::exit(1);
    });
    println!("Searching for {} in {}", config.query, config.filename);

    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)
        .expect("Unable to read file");
    Ok(())
}


struct Config<'a> {
    query: &'a str,
    filename: &'a str
}

impl Config<'_> {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query: &String = &args[1];
        let filename: &String = &args[2];

        Ok(Config { query, filename })
    }
}