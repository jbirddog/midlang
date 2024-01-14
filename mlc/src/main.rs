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
    #[arg(short, long = "library")]
    libraries: Option<Vec<String>>,
    #[arg(short = 'L', long = "library-path")]
    library_paths: Option<Vec<String>>,
    #[arg(short, long)]
    output: Option<String>,
}

const DEFAULT_OUTPUT: &str = "a.out";

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let frontend = json_frontend::new(&args.json_file);

    let libraries = args.libraries.unwrap_or_else(Vec::new);
    let library_paths = args.library_paths.unwrap_or_else(Vec::new);
    let output = args.output.unwrap_or_else(|| DEFAULT_OUTPUT.to_string());
    let backend = qbe_backend::new(&libraries, &library_paths, &output);

    let compiler = compiler::new(&frontend, &backend, &args.build_dir, &args.ninja);

    compiler.compile()
}
