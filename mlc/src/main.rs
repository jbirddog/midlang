use std::error::Error;

use clap::Parser;

use midlang::compiler;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    json_file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let frontend = json_frontend::new();
    let backend = qbe_backend::new();
    let compiler = compiler::new(&frontend, &backend);

    compiler.compile(&args.json_file)
}
