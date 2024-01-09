use std::error::Error;

use clap::Parser;

use json_frontend::Frontend;
use midlang::compiler::Frontend as _;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    json_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let midlang_module = Frontend::parse_file_named(&args.json_file)?;
    let _ = qbe_backend::lower(&midlang_module);

    println!("Parsed {}", args.json_file);

    Ok(())
}
