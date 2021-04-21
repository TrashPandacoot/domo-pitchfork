use serde::{Serialize, de::DeserializeOwned};

/// Return CSV string from a Vec of Records to upload to Domo.
pub fn serialize_to_csv_str<T: Serialize>(
    data: &[T],
    write_headers: bool
) -> Result<String, Box<dyn std::error::Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(write_headers)
        .from_writer(vec![]);
    for record in data {
        wtr.serialize(record)?;
    }
    let csv_str = String::from_utf8(wtr.into_inner()?)?;

    Ok(csv_str)
}

/// Deserialize a CSV string into a Vec
pub fn deserialize_csv_str<T: DeserializeOwned>(csv: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let output: Result<Vec<T>, csv::Error> = rdr.deserialize().collect();
    output.map_err(|e| -> Box<dyn std::error::Error> {Box::new(e)})
}

pub(crate) fn serialize_csv_str<T: Serialize>(
    data: &[T],
    write_headers: bool,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(write_headers)
        .from_writer(vec![]);
    for record in data {
        wtr.serialize(record)?;
    }
    let csv_str = String::from_utf8(
        wtr.into_inner()?
    )?;

    Ok(csv_str)
}