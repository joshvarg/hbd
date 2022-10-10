use std::io;
use std::io::prelude::*;
use std::fs::File;
use clap::Parser;

#[derive(Parser,Default,Debug)]
#[clap(author="Author Name", version, about)]
/// Never be the family asshole again!
struct Arguments {
    name: String,
    day: usize,
    month: usize,
    year: usize,
}


fn entry_add(name: &String, day: usize, month: usize, year: usize) -> io::Result<File> {
    let mut file = File::create("entry.txt").expect("Could not create file");
    let output = format!("{} has a birthday on {}-{}-{}\n", name, month, day, year);
    file.write_all(output.as_bytes()).expect("Could not write entry");

    Ok(file)
}


fn main() {
    let args = Arguments::parse();
    //: Vec<_> = env::args().collect();
    println!("{:?}", args);
    let name = &args.name;
    let day = args.day;
    let month = args.month;
    let year = args.year;
    entry_add(name, day, month, year).expect("Could not add entry");
    
}
