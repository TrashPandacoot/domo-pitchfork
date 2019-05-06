use serde::{Serialize, de::DeserializeOwned};
use std::error::Error;
use crate::error::PitchforkError;

/// Return CSV string from a Vec of Records to upload to Domo.
pub fn serialize_to_csv_str<T: Serialize>(
    data: &[T],
) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
    const WRITE_HEADERS: bool = false;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(WRITE_HEADERS)
        .from_writer(vec![]);
    for record in data {
        wtr.serialize(record)?;
    }
    let csv_str = String::from_utf8(wtr.into_inner()?)?;

    Ok(csv_str)
}

pub fn deserialize_csv_str<T: DeserializeOwned>(csv: &str) -> Result<Vec<T>, PitchforkError> {
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let output: Result<Vec<T>, csv::Error> = rdr.deserialize().collect();
    output.map_err(PitchforkError::from)
}