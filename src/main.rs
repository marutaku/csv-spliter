use std::env;
use std::process;

use crate::split::CSVFragment;

mod split;

fn generate_csv_file_name(file_prefix: &str, index: usize) -> String {
    format!("{}{}.csv", file_prefix, index)
}

fn main() {
    let filepath: String = env::args().nth(1).expect("File path not given");
    let line_count_str: String = env::args().nth(2).expect("Missing line count");
    let file_prefix = "prefix_"; // TODO: 引数として受け取る

    let line_count: usize = line_count_str
        .parse()
        .expect("Line count should be a number");
    let mut reader = match csv::Reader::from_path(&filepath) {
        Err(why) => {
            println!("cannot open {}: {}", &filepath, why);
            process::exit(1);
        }
        Ok(content) => content,
    };
    let header = reader.headers().expect("No header").clone();
    let mut csv_flagment = CSVFragment::new(header.clone(), line_count);
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
            let csv_file_name = generate_csv_file_name(file_prefix, file_iteration_counter);
            csv_flagment
                .write_records_to_file(csv_file_name.as_str())
                .expect("cannnot create file");
            file_iteration_counter += 1;
            csv_flagment = CSVFragment::new(header.clone(), line_count);
        }
    }
    if csv_flagment.len() != 0 {
        csv_flagment
            .write_records_to_file(
                generate_csv_file_name(file_prefix, file_iteration_counter).as_str(),
            )
            .expect("can not create file");
    }
}
