// Domo Dataset Objects
use super::policy::Policy;
use super::user::Owner;

use self::FieldType::*;
use std::collections::HashMap;
use std::error::Error;
use std::str::{self, FromStr};
use serde::{Deserialize, Serialize};

//[Dataset object](https://developer.domo.com/docs/dataset-api-reference/dataset#The%20DataSet%20object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub columns: Option<i32>,
    pub rows: Option<i32>,
    pub schema: Option<Schema>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "dataCurrentAt")]
    pub data_current_at: Option<String>,
    #[serde(rename = "pdpEnabled")]
    pub pdp_enabled: Option<bool>,
    pub owner: Option<Owner>,
    pub policies: Option<Vec<Policy>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatasetSchema {
    pub name: String,
    pub description: String,
    pub rows: u32,
    pub schema: Schema,
}

// TODO: Fix Link
//[Schema Object](https://developer.domo.com/)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "columns")]
    pub columns: Vec<Column>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Column {
    #[serde(rename = "type")]
    pub column_type: String,
    pub name: String,
}

impl DatasetSchema {
    pub fn from_hashmap(
        name: String,
        description: String,
        col_schema: &HashMap<String, FieldType>,
    ) -> DatasetSchema {
        DatasetSchema {
            name,
            description,
            rows: 0,
            schema: Schema::from_hashmap(&col_schema),
        }
    }
}

impl Schema {
    pub fn from_hashmap(cols: &HashMap<String, FieldType>) -> Schema {
        let mut columns: Vec<Column> = Vec::new();
        for (col, typ) in cols {
            let typ_str = match typ {
                TUnicode => "STRING".to_string(),
                TFloat => "DOUBLE".to_string(),
                TInteger => "LONG".to_string(),
                _ => "STRING".to_string(),
            };
            columns.push(Column {
                column_type: typ_str,
                name: col.to_string(),
            })
        }
        Schema { columns }
    }
}

pub enum DomoDataType {
    STRING,
    LONG,
    DECIMAL,
    DOUBLE,
    DATETIME,
    DATE,
}

impl DomoDataType {
    // TODO: document where this is needed
    #[allow(dead_code)]
    fn from_fieldtype(typ: FieldType) -> DomoDataType {
        match typ {
            TNull => DomoDataType::STRING,
            TUnicode => DomoDataType::STRING,
            TInteger => DomoDataType::LONG,
            TFloat => DomoDataType::DECIMAL,
            _ => DomoDataType::STRING,
        }
    }
}

impl From<DomoDataType> for String {
    fn from(domo_type: DomoDataType) -> String {
        match domo_type {
            DomoDataType::STRING => "STRING".to_owned(),
            DomoDataType::LONG => "LONG".to_owned(),
            DomoDataType::DECIMAL => "DOUBLE".to_owned(),
            DomoDataType::DOUBLE => "DOUBLE".to_owned(),
            DomoDataType::DATETIME => "DATETIME".to_owned(),
            DomoDataType::DATE => "DATE".to_owned(),
        }
    }
}

// This introduces a type alias so that we can conveniently reference our
// record type.
pub type Record = HashMap<String, String>;
pub type CsvColumnTypes = HashMap<String, FieldType>;

pub fn check_field_type(rec: &Record, cols: &mut CsvColumnTypes) -> Result<(), Box<Error>> {
    for (key, value) in rec.iter() {
        let typ = FieldType::from_sample(value.as_bytes());
        let cur_typ = cols.entry(key.to_string()).or_insert(typ);
        cur_typ.merge(typ);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldType {
    TUnknown,
    TNull,
    TUnicode,
    TFloat,
    TInteger,
}

impl FieldType {
    pub fn merge(&mut self, other: FieldType) {
        *self = match (*self, other) {
            (TUnicode, TUnicode) => TUnicode,
            (TFloat, TFloat) => TFloat,
            (TInteger, TInteger) => TInteger,
            // Null does not impact the type.
            (TNull, any) | (any, TNull) => any,
            // There's no way to get around an unknown.
            (TUnknown, _) | (_, TUnknown) => TUnknown,
            // Integers can degrade to floats.
            (TFloat, TInteger) | (TInteger, TFloat) => TFloat,
            // Numbers can degrade to Unicode strings.
            (TUnicode, TFloat) | (TFloat, TUnicode) => TUnicode,
            (TUnicode, TInteger) | (TInteger, TUnicode) => TUnicode,
        };
    }

    pub fn from_sample(sample: &[u8]) -> FieldType {
        if sample.is_empty() {
            return TNull;
        }
        let string = match str::from_utf8(sample) {
            Err(_) => return TUnknown,
            Ok(s) => s,
        };
        if string.parse::<i64>().is_ok() {
            return TInteger;
        }
        if string.parse::<f64>().is_ok() {
            return TFloat;
        }
        TUnicode
    }

    pub fn is_number(self) -> bool {
        self == TFloat || self == TInteger
    }

    pub fn is_null(self) -> bool {
        self == TNull
    }
}

impl Default for FieldType {
    // The default is the most specific type.
    // Type inference proceeds by assuming the most specific type and then
    // relaxing the type as counter-examples are found.
    fn default() -> FieldType {
        TNull
    }
}

// TODO: Check if this is actually needed
#[allow(dead_code)]
fn from_bytes<T: FromStr>(bytes: &[u8]) -> Option<T> {
    str::from_utf8(bytes).ok().and_then(|s| s.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_dataset_schema_serialization() {
        let c = Column {
            column_type: "STRING".to_string(),
            name: "column name".to_string(),
        };
        let s = Schema { columns: vec![c] };
        let d_schema = DatasetSchema {
            name: "test dataset".to_string(),
            description: "test description".to_string(),
            rows: 0,
            schema: s,
        };
        let expected = json!({
            "name": "test dataset",
            "description": "test description",
            "rows": 0,
            "schema": {
                "columns": [{
                    "type": "STRING",
                    "name": "column name"
                }]
            },
        });

        let v = serde_json::to_value(d_schema).unwrap();
        assert_eq!(v, expected);
    }

    #[test]
    fn test_fieldtype_merge() {
        assert!(false);
    }

    #[test]
    fn test_fieldtype_from_sample() {
        assert!(false);
    }

    #[test]
    fn test_check_fieldtype() {
        assert!(false);
    }

    #[test]
    fn test_schema_from_hashmap() {
        assert!(false);
    }

    #[test]
    fn test_datasetschema_from_hashmap() {
        assert!(false);
    }

}
