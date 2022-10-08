use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;


fn entry_add(name: &String, dob: &String) -> io::Result<File> {
    let mut file = File::create("entry.txt").expect("Could not create file");
    let output = format!("{} {}", name, dob);
    file.write_all(output.as_bytes()).expect("Could not write entry");

    Ok(file)
}


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("app <name> <DoB>");
    } else {
        let name = &args[1];
        let dob = &args[2];
        entry_add(name, dob).expect("Could not add entry");
    }
}
