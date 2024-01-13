use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    json_file: String,
    #[arg(short, long)]
    build_dir: String,
    #[arg(short, long)]
    ninja: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let frontend = json_frontend::new(&args.json_file);
    let backend = qbe_backend::new();
    let compiler = compiler::new(&frontend, &backend, &args.build_dir, &args.ninja);

    compiler.compile()
}
