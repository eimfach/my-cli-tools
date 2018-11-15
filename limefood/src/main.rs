#[macro_use] extern crate structopt;
extern crate failure;
extern crate exitfailure;

use std::{
    fs,
    path::{PathBuf},
};

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

// Our CLI arguments. (help and version are automatically generated)
// Documentation on how to use:
// https://docs.rs/structopt/0.2.10/structopt/index.html#how-to-derivestructopt
#[derive(StructOpt)]
struct Cli {
    // The path of the file we want to look at.
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: PathBuf,
    #[structopt(short = "l", long = "length")]
    length: usize
}

fn main() -> Result<(), ExitFailure>{
    let cli = Cli::from_args();
    let Cli { path, length } = &cli;

    let contents = fs::read_to_string(path)
        .with_context(|_| format!("Could not read file {}", path.to_str().unwrap()))?;

    let mut out = String::new();
    let chars = contents.chars();
    let mut i = 1;
    for char_item in chars {
        if char_item != '\n' {
            out.push(char_item);
        }
 
        if i % length == 0 {
            out.push('\n');
        }

        i = i + 1;
    }
    fs::write(path, out)?;
    Ok(())
}

