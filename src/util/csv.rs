use crate::error::DomoError;
use serde::Serialize;
/// Return CSV string from a Vec of Records to upload to Domo.
pub fn serialize_to_csv_str<T: Serialize>(data: &[T]) -> Result<String, DomoError> {
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