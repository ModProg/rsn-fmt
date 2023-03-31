use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Opts {
    file: PathBuf,
}

fn main() {
    let opts = Opts::parse();
    fs::write(
        &opts.file,
        rsn_fmt::format_str(&fs::read_to_string(&opts.file).unwrap()),
    )
    .unwrap();
}
