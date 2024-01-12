use std::error::Error;
use glob::glob;

pub struct Config {
    pub file: String,
}

impl Config {
    /// Builds a Config struct from commandline arguments
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::build(env::args()).unwrap_or_else(|err| {
    ///     eprintln!("Problem parsing arguments: {err}");
    ///     process::exit(1);
    /// });
    /// ```
    pub fn build(
        mut args: impl Iterator<Item=String>,
    ) -> Result<Config, &'static str> {

        args.next();

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file string"),
        };

        Ok(Config { file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config.file);
    walk_directories().unwrap();
    Ok(())
}

fn walk_directories() -> Result<(), Box<dyn Error>> {
    for entry in glob("**/*.rs")? {
        println!("{}", entry?.display());
    }

    Ok(())
}











