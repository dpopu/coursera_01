use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Name of the input file
    #[arg(long)]
    input_file_name: String,
}

fn main() {
    let args = Args::parse();
    println!("Input file: {}", args.input_file_name);

    let file = File::open(args.input_file_name);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
