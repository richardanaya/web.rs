use clap::Parser;
use std::fs::File;
use std::io::Read;
mod typescript;
use typescript::parse;
mod generation;
use generation::generate_bindings;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: String,
}

fn main() {
    let args = Args::parse();

    // open file and get bytes
    let input_file = args.input_file;

    let mut file = File::open(input_file).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let result = parse(&buffer);
    if let Ok(definition_file) = result {
        generate_bindings(&definition_file);
        println!("Done!");
    } else {
        println!("Error generating bindings: {:?}", result);
    }
}
