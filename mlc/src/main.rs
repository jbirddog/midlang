use std::error::Error;

use clap::Parser;

use json_frontend::{lower, parse_file_named};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    json_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let json_module = parse_file_named(&args.json_file)?;
    let _midlang_module = lower(&json_module)?;

    println!("Parsed {}", args.json_file);

    Ok(())
}
