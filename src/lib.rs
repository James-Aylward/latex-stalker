use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{thread, time};
use std::process::Command;
use std::collections::HashMap;
use glob::glob;

pub struct Config {
    pub file: String,
    pub output_file: String,
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
        let output_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an output file"),
        };

        Ok(Config { file, output_file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let mut old_index = index_files()?;
    let _ = compile_latex(&config.file);
    let mupdf_handle = Command::new("mupdf")
        .arg(&config.output_file)
        .spawn()?;

    loop {
        thread::sleep(time::Duration::from_millis(50));
        let new_index = index_files()?;
        if old_index != new_index {
            println!("Refreshing!");
            let _ = compile_latex(&config.file);
            refresh_viewer(mupdf_handle.id());
            old_index = new_index;
        }
    }
}

fn compile_latex(file: &String) -> Result<(), Box<dyn Error>>{
    let _pdflatex_command = Command::new("pdflatex")
        .arg("-halt-on-error")
        .arg("-shell-escape")
        .arg("-interaction=batchmode")
        .arg(file)
        .spawn()?
        .wait();
    Ok(())
}

fn refresh_viewer(id: u32) {
    let _refresh_command = Command::new("kill")
        .arg("-HUP")
        .arg(format!("{}", id))
        .output();
}

fn index_files() -> Result<HashMap<PathBuf, Vec<u8>>, Box<dyn Error>> {
    let mut footprint: HashMap<PathBuf, Vec<u8>> = HashMap::new();
    for entry in glob("**/*.tex")? {
        if let Ok(pathbuf) = entry {
            let mut f = File::open(&pathbuf)?;
            let mut b = Vec::new();
            f.read_to_end(&mut b)?;
            footprint.insert(pathbuf, b);
        }
    }

    Ok(footprint)
}
