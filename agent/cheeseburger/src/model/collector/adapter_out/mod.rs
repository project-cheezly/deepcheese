pub(crate) mod csv_writer;

use crate::model::collector::response::StreamSerializer;

pub use csv_writer::CsvCollector;

pub trait DataCollector<T> where T: StreamSerializer {
    fn write(&mut self, data: T::Input) -> Result<(), Box<dyn std::error::Error>>;
}