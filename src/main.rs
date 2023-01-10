use clap::Parser;

mod fragment;
mod split;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_filepath: String,
    line_count: usize,
    #[arg(short, long)]
    prefix: Option<String>,
    #[arg(short, long)]
    output_dir: Option<String>,
}

fn main() {
    let args = Args::parse();
    let file_prefix = args.prefix.unwrap_or(String::from("./"));

    split::split_csv(args.input_filepath, args.line_count, &file_prefix)
        .expect("Split csv failed: ");
}
