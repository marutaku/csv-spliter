use std::env;

use clap::Parser;

mod fragment;
mod split;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_filename: String,
    line_count: usize,
    #[arg(short, long)]
    prefix: Option<String>,
    #[arg(short, long)]
    output_dir: Option<String>,
}

fn main() {
    let filepath: String = env::args().nth(1).expect("File path not given");
    let line_count_str: String = env::args().nth(2).expect("Missing line count");
    let line_count: usize = line_count_str
        .parse()
        .expect("Line count should be a number");
    let file_prefix = "prefix_"; // TODO: 引数として受け取る

    split::split_csv(filepath, line_count, file_prefix).expect("Split csv failed: ");
}
