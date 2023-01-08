use std::env;
use std::process;

use crate::split::CSVFragment;

mod split;

fn main() {
    let filepath: String = env::args().nth(1).expect("File path not given");
    let line_count_str: String = env::args().nth(2).expect("Missing line count");
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
            let csv_file_name = format!("hoge_{}.csv", &file_iteration_counter);
            csv_flagment
                .write_records_to_file(csv_file_name.as_str())
                .expect(format!("cannnot create {csv_file_name}").as_str());
            file_iteration_counter += 1;
            csv_flagment = CSVFragment::new(header.clone(), line_count);
        }
    }
    println!("line count: {line_count}");
}
