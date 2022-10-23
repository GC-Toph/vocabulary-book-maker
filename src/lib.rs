mod filters;

use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;

pub struct Config {
    pub filename: String,
    pub is_shuffle: bool,
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Please input one argument as filename! "),
        };

        let is_shuffle = env::var("SHUFFLE").is_ok();

        Ok(Config {
            filename,
            is_shuffle,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let origin_text = fs::read_to_string(config.filename)?;

    let high_freq_words = filters::main_filter(origin_text)?;
    let mut file = fs::File::create("out_2_main.txt")?;
    file.write_all(high_freq_words.join("\n").as_bytes())?;

    let after_exclude = filters::exclude::exclude(high_freq_words)?;
    let mut file = fs::File::create("out_3_ex.txt")?;
    file.write_all(after_exclude.join("\n").as_bytes())?;

    let mut after_hamming = filters::exclude::exclude_hamming(after_exclude)?;
    let mut file = fs::File::create("out_4_ham.txt")?;
    file.write_all(after_hamming.join("\n").as_bytes())?;

    if config.is_shuffle {
        let after_shuffle = filters::shuffle(&mut after_hamming);
        let mut file = fs::File::create("out_5_shuffle.txt")?;
        file.write_all(after_shuffle.join("\n").as_bytes())?;
    }
    Ok(())
}
