use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use clap::{Parser, Subcommand};
// TODO: use 'dirs' crate for actual path finding

#[derive(Parser)]
#[clap(author="Author Name", version, about)]
/// Never be the family asshole again!
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        year: usize,
        month: usize,
        day: usize,
    },
    List,
}


fn entry_add(name: &String, day: &usize, month: &usize, year: &usize) -> io::Result<File> {
    fs::create_dir_all("~/.bday/");
    // TODO: fix this path to be the actual home of user
    let mut file = File::create("~/.bday/entry.txt").expect("Could not create file");
    let output = format!("{} has a birthday on {}-{}-{}\n", name, *month, *day, *year);
    file.write_all(output.as_bytes()).expect("Could not write entry");

    Ok(file)
}

fn list() -> String {
    let contents = fs::read_to_string("~/.bday/entry.txt").expect("No birthdays logged in ~/.bday/ !");
    // TODO: fix this path to be the actual home of user
    return contents;
}


fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { name, year, month, day } => {
            entry_add(name, day, month, year).expect("Could not add entry");
        }
        Commands::List => {
            println!("{}", list());
        }
    }
}
