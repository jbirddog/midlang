use std::error::Error;

use clap::Parser;

use json_frontend::{lower, parse_file_named, LoweringCtx};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    json_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let json_module = parse_file_named(&args.json_file)?;
    let mut ctx: LoweringCtx = Default::default();
    let _midlang_module = lower(&json_module, &mut ctx)?;

    println!("Parsed {}", args.json_file);

    Ok(())
}
