extern crate colored;
extern crate structopt;

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    ///What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    ///Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    ///lLoad the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    ///Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    // let message = options.message;
    let mut message = String::new();
    let eye = if options.dead { "x" } else { "o" };
    // let message = std::env::args()
    //     .nth(1)
    //     .expect("Missing the message. Usage: MrCat <message>");

    if message.to_ascii_uppercase() == "woof" {
        eprintln!("A cat shouldnt bark like a dog.")
    }

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    };

    println!("{}", message.bright_yellow().underline().on_purple());
    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", cat_picture);
        }
        None => {
            println!(" \\");
            println!("  \\");
            println!("  /\\_/\\");
            println!("  ( {eye} {eye} )", eye = eye.red().bold());
            println!("  =( I )=");
        }
    }
    Ok(())
}
