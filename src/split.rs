use std::error::Error;
use std::process;

use crate::fragment;

fn generate_csv_file_name(file_prefix: &str, index: usize) -> String {
    format!("{}{}.csv", file_prefix, index)
}

pub fn split_csv(
    filepath: String,
    line_count: usize,
    file_prefix: &str,
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
            let csv_file_name = generate_csv_file_name(file_prefix, file_iteration_counter);
            csv_flagment
                .write_records_to_file(csv_file_name.as_str())
                .expect("cannnot create file");
            file_iteration_counter += 1;
            csv_flagment = fragment::CSVFragment::new(header.clone(), line_count);
        }
    }
    if csv_flagment.len() != 0 {
        csv_flagment
            .write_records_to_file(
                generate_csv_file_name(file_prefix, file_iteration_counter).as_str(),
            )
            .expect("can not create file");
    }
    Ok(())
}
