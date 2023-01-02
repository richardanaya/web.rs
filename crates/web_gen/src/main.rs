use clap::Parser;
use std::fs::File;
use std::io::Read;
mod typescript;
use typescript::parse;
mod generation;
use generation::generate_bindings;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: String,

    #[clap(short, long)]
    output_dir: Option<String>,
}

fn main() {
    let args = Args::parse();

    // open file and get bytes
    let input_file = args.input_file;

    let mut file = File::open(input_file).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let output_dir: PathBuf = args.output_dir.unwrap_or(".".into()).into();

    let result = parse(&buffer);
    if let Ok(definition_file) = result {
        generate_bindings(&definition_file, output_dir);
        println!("Done!");
    } else {
        println!("Error generating bindings: {:?}", result);
    }
}
