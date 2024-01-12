use std::error::Error;

pub struct Config {
    pub file: String,
}

impl Config {
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
    Ok(())
}
