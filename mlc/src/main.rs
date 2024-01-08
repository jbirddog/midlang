use std::error::Error;

use clap::Parser;

use json_frontend;
use qbe_backend;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    json_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let json_module = json_frontend::parse_file_named(&args.json_file)?;
    let midlang_module = json_frontend::lower(&json_module)?;
    let _ = qbe_backend::lower(&midlang_module);

    println!("Parsed {}", args.json_file);

    Ok(())
}
