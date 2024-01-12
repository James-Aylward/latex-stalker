#![allow(unused)]
use std::any::Any;
use std::error::Error;
use std::process::Command;
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
    ///
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

    let tex_files = glob("**/*.rs")?;

    let mut mupdf_instance = Command::new("mupdf")
        .arg(config.file)
        .spawn()?;
    println!("{}", mupdf_instance.id());

    Ok(())
}
