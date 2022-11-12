use clap::Parser;

/// ARML-engine: CV/resume document generation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Markup File to process
    #[arg(short, long)]
    file: String,

    /// Output Format
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    println!("file: {}\noutput format: {}", args.file, args.output)
}
