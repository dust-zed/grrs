use std::{error::Error, fs::File, io::{self, BufRead, BufReader}, path::PathBuf};

use anyhow::Context;
use clap::Parser;

///search for a pattern in a file and display lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf, 
}
fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("could not read file {:?}", args.path))?;
    let reader = BufReader::new(file);

    let stdout = io::stdout();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
    Ok(())
}

 