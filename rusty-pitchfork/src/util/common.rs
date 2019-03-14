//! utils function
use flate2;
use self::flate2::bufread::GzEncoder;
use self::flate2::Compression;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::prelude::Read;
use std::io::BufReader;
use std::string::ToString;
use chrono::{Utc, DateTime};

pub fn datetime_to_timestamp(elapsed: u32) -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp() + i64::from(elapsed)
}
/// convert map to query string.
/// ex. convert
/// `{"limit":"2", "offset":"4"}`
/// to
/// `limit=2
#[allow(clippy::implicit_hasher)]
pub fn convert_map_to_string<K: Debug + Eq + Hash + ToString, V: Debug + ToString>(
    map: &HashMap<K, V>,
) -> String {
    let mut string: String = String::new();
    for (key, value) in map.iter() {
        if !string.is_empty() {
            string.push_str("&");
        }
        string.push_str(&key.to_string());
        string.push_str("=");
        string.push_str(&value.to_string());
    }
    string
}

pub fn gzip_csv(path: &str) -> io::Result<Vec<u8>> {
    let f = File::open(path)?;
    let b = BufReader::new(f);
    let mut gz = GzEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn gzip_str(data: &str) -> Vec<u8> {
    let b = BufReader::new(data.as_bytes());
    let mut gz = GzEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer)
        .expect("Error Compressing String");
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::flate2::read::GzDecoder;
    // use std::io::prelude::*;

    #[test]
    fn test_gzip_str() {
        let s = "Build Intelligence".to_string();
        let gzip_s = gzip_str(&s);
        let mut actual_s = Vec::new();
        let mut decoder = GzDecoder::new(gzip_s.as_slice());
        decoder.read_to_end(&mut actual_s).unwrap();
        assert_eq!(actual_s, s.as_bytes());
    }

    #[test]
    fn test_gzip_csv() {
        //TODO: implement test
        panic!();
    }

    #[test]
    fn test_convert_map_to_string() {
        let mut map = HashMap::new();
        map.insert("limit".to_string(), 1);
        map.insert("offset".to_string(), 2);
        let actual = convert_map_to_string(&map);
        let expected = format!("limit={}&offset={}", 1, 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_datatime_to_timestamp() {
        //TODO: implement test
        panic!();
    }

}
