#![allow(unused)]
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{thread, time};
use std::process::Command;
use std::collections::{hash_map, HashMap};
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
    index_files()?;

    let mut mupdf_handle = Command::new("mupdf")
        .arg(config.file)
        .spawn()?;

    //thread::sleep(time::Duration::from_secs(1));

    let refresh_command = Command::new("kill")
        .arg("-HUP")
        .arg(format!("{}", mupdf_handle.id()))
        .output();

    Ok(())
}

fn index_files() -> Result<HashMap<PathBuf, Vec<u8>>, Box<dyn Error>> {
    let mut footprint: HashMap<PathBuf, Vec<u8>> = HashMap::new();

    for entry in glob("**/*.rs")? {
        if let Ok(pathbuf) = entry {
            let mut f = File::open(&pathbuf)?;
            let mut b = Vec::new();
            f.read_to_end(&mut b)?;
            footprint.insert(pathbuf, b);
        }
    }

    Ok(footprint)
}

fn updates_made() -> Result<bool, Box<dyn Error>> {
    Ok(true)
}
