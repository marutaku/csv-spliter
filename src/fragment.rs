use csv::{StringRecord, Writer};
use std::{error::Error, path::PathBuf};

pub struct CSVFragment {
    header: StringRecord,
    records: Vec<StringRecord>,
    capacity: usize,
}

impl CSVFragment {
    pub fn new(header: StringRecord, capacity: usize) -> Self {
        CSVFragment {
            header: header.clone(),
            records: Vec::<StringRecord>::with_capacity(capacity),
            capacity,
        }
    }

    pub fn write_records_to_file(&self, filepath: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::from_path(filepath)?;
        writer.write_record(&self.header)?;
        for record in &self.records {
            writer.write_record(record)?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn push(&mut self, row: StringRecord) {
        self.records.push(row);
    }

    pub fn is_full(&self) -> bool {
        self.records.len() == self.capacity
    }
    pub fn len(&self) -> usize {
        self.records.len()
    }
}
