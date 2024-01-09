use std::error::Error;

use clap::Parser;

use midlang::compiler::Frontend;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    json_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let frontend = json_frontend::new();
    let midlang_module = frontend.parse_file_named(&args.json_file)?;
    let _ = qbe_backend::lower(&midlang_module);

    println!("Parsed {}", args.json_file);

    Ok(())
}
