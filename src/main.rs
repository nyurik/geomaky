use std::path::PathBuf;
use clap::Parser;

/// A utility to publish a large set of geojson files on a static host
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the directory containing the geojson files
    data_dir: PathBuf,

    /// Optional path to the optional directory
    #[arg(default_value = "./output")]
    output_dir: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
