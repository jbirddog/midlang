use std::error::Error;

use clap::Parser;

use json_frontend::parse_file_named;
//use midlang;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    json_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let _module = parse_file_named(&args.json_file)?;
    
    println!("Parsed {}", args.json_file);

    Ok(())
}
