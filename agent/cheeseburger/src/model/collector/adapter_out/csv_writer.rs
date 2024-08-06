use std::fs::File;
use csv::{Writer, WriterBuilder};
use crate::model::collector::adapter_out::DataCollector;
use crate::model::collector::response::StreamSerializer;

pub struct CsvCollector<T> {
    writer: Writer<File>,
    _marker: std::marker::PhantomData<T>
}

impl<T> CsvCollector<T> {
    fn get_file_name(query_code: &str, stock_code: &str) -> String {
        let current_date = chrono::Local::now().format("%Y%m%d");

        if !std::path::Path::new("./data").exists() {
            if let Err(e) = std::fs::create_dir("./data") {
                panic!("Failed to create directory: {}", e);
            }
        }

        format!("./data/{}_{}_{}.csv", query_code, stock_code, current_date)
    }
}

impl<T> CsvCollector<T>
where
    T: StreamSerializer
{
    pub fn new(query_code: &str, stock_code: &str)
        -> Result<Self, Box<dyn std::error::Error + Sync + Send>>
    {
        let file_name = Self::get_file_name(query_code, stock_code);
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_path(file_name)
            .or_else(|e| {
                tracing::error!("Failed to create csv file: {}", e);
                Err(e)
            })?;

        writer.write_record(&T::get_headers())?;
        Ok(Self {
            writer,
            _marker: std::marker::PhantomData,
        })
    }
}

impl<T> DataCollector<T> for CsvCollector<T>
where
    T: StreamSerializer
{
    fn write(&mut self, data: T::Input) -> Result<(), Box<dyn std::error::Error>> {
        self.writer.serialize(T::from(data))?;
        self.writer.flush()?;
        Ok(())
    }
}
