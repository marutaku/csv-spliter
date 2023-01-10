use crate::fragment;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process;

fn generate_csv_file_name(file_prefix: &str, index: usize) -> String {
    format!("{}{}.csv", file_prefix, index)
}

fn build_output_filepath(output_dir: &str, filename: &str) -> PathBuf {
    Path::new(output_dir).join(filename)
}

pub fn split_csv(
    filepath: String,
    line_count: usize,
    file_prefix: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let mut reader = match csv::Reader::from_path(&filepath) {
        Err(why) => {
            println!("cannot open {}: {}", &filepath, why);
            process::exit(1);
        }
        Ok(content) => content,
    };
    let header = reader.headers().expect("No header").clone();
    let mut csv_flagment = fragment::CSVFragment::new(header.clone(), line_count);
    let mut file_iteration_counter = 1;
    for result in reader.records() {
        let record = match result {
            Err(error) => {
                println!("wrong csv format {}", error);
                process::exit(1)
            }
            Ok(record) => record,
        };
        csv_flagment.push(record);
        if csv_flagment.is_full() {
            let csv_filename = generate_csv_file_name(file_prefix, file_iteration_counter);
            let csv_filepath = build_output_filepath(output_dir, &csv_filename);
            csv_flagment
                .write_records_to_file(&csv_filepath)
                .expect("cannnot create file");
            file_iteration_counter += 1;
            csv_flagment = fragment::CSVFragment::new(header.clone(), line_count);
        }
    }
    if csv_flagment.len() != 0 {
        let csv_filename = generate_csv_file_name(file_prefix, file_iteration_counter);
        let csv_filepath = build_output_filepath(output_dir, &csv_filename);
        csv_flagment
            .write_records_to_file(&csv_filepath)
            .expect("can not create file");
    }
    Ok(())
}
