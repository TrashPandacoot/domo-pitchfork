//! Domo Dataset API
//!
//! # [`DatasetsRequestBuilder`](`crate::pitchfork::DatasetsRequestBuilder`) implements all available dataset API endpoints and functionality
//!
//! Additional Resources:
//! - [Domo Dataset API Reference](https://developer.domo.com/docs/dataset-api-reference/dataset)
use super::policy::Policy;
use super::user::Owner;
use crate::util::csv::{deserialize_csv_str, serialize_to_csv_str};
use chrono::FixedOffset;
use serde_json::json;
use serde_json::Value;

use crate::error::{PitchforkError, PitchforkErrorKind};
use crate::pitchfork::{DatasetsRequestBuilder, DomoRequest};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use log::debug;
use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

impl<'t> DatasetsRequestBuilder<'t, Dataset> {
    /// Retreives details for a `Dataset`
    ///
    /// # Example
    /// ```no_run
    /// # use domo_pitchfork::error::PitchforkError;
    /// use domo_pitchfork::pitchfork::DomoPitchfork;
    /// let domo = DomoPitchfork::with_token("token");
    /// let dataset_info = domo.datasets().info("dataset id")?;
    /// println!("Dataset Details: \n{:#?}", dataset_info);
    /// # Ok::<(), PitchforkError>(())
    /// ```
    ///
    pub fn info(mut self, dataset_id: &str) -> Result<Dataset, PitchforkError> {
        self.url.push_str(dataset_id);
        let req = Self {
            method: Method::GET,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        req.retrieve_and_deserialize_json()
    }

    /// List Datasets starting from a given offset up to a given limit.
    /// Max limit is 50.
    /// # Example
    /// ```no_run
    /// # use domo_pitchfork::error::PitchforkError;
    /// use domo_pitchfork::pitchfork::DomoPitchfork;
    /// let domo = DomoPitchfork::with_token("token");
    /// let dataset_list = domo.datasets().list(5,0)?;
    /// dataset_list.iter().map(|ds| println!("Dataset Name: {}", ds.name.as_ref().unwrap()));
    /// # Ok::<(),PitchforkError>(())
    /// ```
    pub fn list(mut self, limit: u32, offset: u32) -> Result<Vec<Dataset>, PitchforkError> {
        // TODO: impl sort optional query param
        self.url
            .push_str(&format!("?limit={}&offset={}", limit, offset));
        let req = Self {
            method: Method::GET,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        let ds_list = serde_json::from_reader(req.send_json()?)?;
        Ok(ds_list)
    }

    /// Create a new empty Domo Dataset.
    pub fn create(self, ds_meta: &DatasetSchema) -> Result<Dataset, PitchforkError> {
        let body = serde_json::to_string(ds_meta)?;
        debug!("body: {}", body);
        let req = Self {
            method: Method::POST,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(body),
        };
        req.retrieve_and_deserialize_json()
    }

    /// Delete the dataset for the given id.
    /// This is destructive and cannot be reversed.
    /// # Example
    /// ```no_run
    /// # use domo_pitchfork::pitchfork::DomoPitchfork;
    /// # let token = "token_here";
    /// let domo = DomoPitchfork::with_token(&token);
    ///
    /// // if it fails to delete print err msg.
    /// if let Err(e) = domo.datasets().delete("ds_id") {
    ///     println!("{}", e)
    /// }
    /// ```
    pub fn delete(mut self, dataset_id: &str) -> Result<(), PitchforkError> {
        self.url.push_str(dataset_id);
        let req = Self {
            method: Method::DELETE,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        req.send_json()?;
        Ok(())
    }

    /// Modify an existing Domo Dataset.
    pub fn modify(
        mut self,
        dataset_id: &str,
        ds_meta: &DatasetSchema,
    ) -> Result<Dataset, PitchforkError> {
        self.url.push_str(dataset_id);
        let body = serde_json::to_string(ds_meta)?;
        debug!("body: {}", body);
        let req = Self {
            method: Method::PUT,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(body),
        };
        let ds = serde_json::from_reader(req.send_json()?)?;
        Ok(ds)
    }

    /// Returns data from the DataSet based on a SQL query.
    /// # Example
    /// ```no_run
    /// # use domo_pitchfork::pitchfork::DomoPitchfork;
    /// # let token = "token_here";
    /// let domo = DomoPitchfork::with_token(&token);
    /// let dq = domo.datasets()
    ///             .query_data("ds_id", "SELECT * FROM table");
    /// match dq {
    ///     Ok(query_result) => {
    ///         println!("{:#?}", query_result);
    ///     },
    ///     Err(e) => println!("{}", e),
    /// };
    /// ```
    /// [Domo Dataset API Query Reference](https://developer.domo.com/docs/dataset-api-reference/dataset#Query%20a%20DataSet)
    pub fn query_data(
        mut self,
        dataset_id: &str,
        sql_query: &str,
    ) -> Result<DatasetQueryData, PitchforkError> {
        self.url.push_str(&format!("query/execute/{}", dataset_id));
        let body = json!({ "sql": sql_query });
        let req = Self {
            method: Method::POST,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(body.to_string()),
        };
        let dq = serde_json::from_reader(req.send_json()?)?;
        Ok(dq)
    }

    /// Retrieve data from a Domo Dataset as a csv string.
    pub fn download_data(
        mut self,
        dataset_id: &str,
        include_csv_headers: bool,
    ) -> Result<String, PitchforkError> {
        self.url.push_str(&format!(
            "{}/data?includeHeader={}",
            dataset_id, include_csv_headers
        ));
        self.send_json()?.text().map_err(PitchforkError::from)
    }

    /// Retrieve data from a Domo Dataset and Deserialize the retrieved data into a Vec<T>.
    pub fn get_data<T: DeserializeOwned>(
        mut self,
        dataset_id: &str,
    ) -> Result<Vec<T>, PitchforkError> {
        self.url
            .push_str(&format!("{}/data?includeHeader=true", dataset_id));
        deserialize_csv_str(&self.send_json()?.text().map_err(PitchforkError::from)?)
    }

    /// Upload data to the Domo Dataset.
    pub fn upload_from_str(
        mut self,
        dataset_id: &str,
        data_rows: String,
    ) -> Result<(), PitchforkError> {
        self.url.push_str(&format!("{}/data", dataset_id));
        let req = Self {
            method: Method::PUT,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(data_rows),
        };
        req.send_csv()?;
        Ok(())
    }

    /// Upload data to the Domo Dataset.
    pub fn upload_serializable<T: Serialize>(
        mut self,
        dataset_id: &str,
        data: &[T],
    ) -> Result<(), PitchforkError> {
        if data.is_empty() {
            Err(PitchforkError::new("data is empty").with_kind(PitchforkErrorKind::Unknown))
        }
        self.url.push_str(&format!("{}/data", dataset_id));
        let req = Self {
            method: Method::PUT,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(
                serialize_to_csv_str(&data, false)
                    .map_err(|e| PitchforkError::from(e).with_kind(PitchforkErrorKind::Csv))?,
            ),
        };
        req.send_csv()?;
        Ok(())
    }

    /// Retrieves details of a given policy for a Dataset
    pub fn pdp_policy_info(
        mut self,
        dataset_id: &str,
        policy_id: u32,
    ) -> Result<Policy, PitchforkError> {
        self.url
            .push_str(&format!("{}/policies/{}", dataset_id, policy_id));
        let req = Self {
            method: Method::GET,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        let dq = serde_json::from_reader(req.send_json()?)?;
        Ok(dq)
    }

    /// Add a new PDP Policy to a dataset.
    pub fn add_pdp_policy(
        mut self,
        dataset_id: &str,
        policy: &Policy,
    ) -> Result<Policy, PitchforkError> {
        self.url.push_str(&format!("{}/policies", dataset_id));
        let body = serde_json::to_string(policy)?;
        debug!("body: {}", body);
        let req = Self {
            method: Method::POST,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(body),
        };
        let ds = serde_json::from_reader(req.send_json()?)?;
        Ok(ds)
    }

    /// Modify an existing PDP Policy on a dataset.
    pub fn modify_pdp_policy(
        mut self,
        dataset_id: &str,
        policy_id: u32,
        policy: &Policy,
    ) -> Result<Policy, PitchforkError> {
        self.url
            .push_str(&format!("{}/policies/{}", dataset_id, policy_id));
        let body = serde_json::to_string(policy)?;
        debug!("body: {}", body);
        let req = Self {
            method: Method::PUT,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(body),
        };
        let ds = serde_json::from_reader(req.send_json()?)?;
        Ok(ds)
    }

    /// Delete a PDP policy from a Dataset
    pub fn delete_pdp_policy(
        mut self,
        dataset_id: &str,
        policy_id: u32,
    ) -> Result<(), PitchforkError> {
        self.url
            .push_str(&format!("{}/policies/{}", dataset_id, policy_id));
        let req = Self {
            method: Method::DELETE,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        req.send_json()?;
        Ok(())
    }

    /// Retrieves a list of all policies for a Dataset
    pub fn policies(mut self, dataset_id: &str) -> Result<Vec<Policy>, PitchforkError> {
        self.url.push_str(&format!("{}/policies", dataset_id));
        let req = Self {
            method: Method::GET,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        let dq = serde_json::from_reader(req.send_json()?)?;
        Ok(dq)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatasetQueryData {
    pub datasource: String,
    pub columns: Vec<String>,
    pub metadata: Vec<DataQueryMetadata>,
    pub rows: Vec<Vec<Value>>, // Array of Arrays
    #[serde(rename = "numRows")]
    pub num_rows: u64,
    #[serde(rename = "numColumns")]
    pub num_columns: u16,
    #[serde(rename = "fromcache")]
    pub from_cache: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataQueryMetadata {
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(rename = "dataSourceId")]
    pub data_source_id: String,
    #[serde(rename = "maxLength")]
    pub max_length: i32,
    #[serde(rename = "minLength")]
    pub min_length: i32,
    #[serde(rename = "periodIndex")]
    pub period_index: i32,
}
///[Dataset object](https://developer.domo.com/docs/dataset-api-reference/dataset#The%20DataSet%20object)
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
///[Schema Object](https://developer.domo.com/)
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
    ) -> Self {
        Self {
            name,
            description,
            rows: 0,
            schema: Schema::from_hashmap(&col_schema),
        }
    }
}

impl Schema {
    pub fn from_hashmap(cols: &HashMap<String, FieldType>) -> Self {
        let mut columns: Vec<Column> = Vec::new();
        for (col, typ) in cols {
            let typ_str = DomoDataType::from_fieldtype(*typ).to_string();
            columns.push(Column {
                column_type: typ_str,
                name: col.to_string(),
            })
        }
        Self { columns }
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
    pub fn from_fieldtype(typ: FieldType) -> Self {
        match typ {
            FieldType::TNull | FieldType::TUnknown | FieldType::TUnicode => DomoDataType::STRING,
            FieldType::TInteger => DomoDataType::LONG,
            FieldType::TFloat => DomoDataType::DECIMAL,
            FieldType::TDateTime => DomoDataType::DATETIME,
            FieldType::TDate => DomoDataType::DATE,
        }
    }
}

impl From<DomoDataType> for String {
    fn from(domo_type: DomoDataType) -> Self {
        match domo_type {
            DomoDataType::STRING => "STRING".to_owned(),
            DomoDataType::LONG => "LONG".to_owned(),
            DomoDataType::DECIMAL => "DECIMAL".to_owned(),
            DomoDataType::DOUBLE => "DOUBLE".to_owned(),
            DomoDataType::DATETIME => "DATETIME".to_owned(),
            DomoDataType::DATE => "DATE".to_owned(),
        }
    }
}

impl fmt::Display for DomoDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomoDataType::STRING => write!(f, "STRING"),
            DomoDataType::LONG => write!(f, "LONG"),
            DomoDataType::DECIMAL => write!(f, "DECIMAL"),
            DomoDataType::DOUBLE => write!(f, "DOUBLE"),
            DomoDataType::DATETIME => write!(f, "DATETIME"),
            DomoDataType::DATE => write!(f, "DATE"),
        }
    }
}

// This introduces a type alias so that we can conveniently reference our
// record type.
pub type Record = HashMap<String, String>;
pub type CsvColumnTypes = HashMap<String, FieldType>;

pub fn check_field_type(rec: &Record, cols: &mut CsvColumnTypes) -> Result<(), Box<dyn Error>> {
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
    TDate,
    TDateTime,
}

impl FieldType {
    pub fn merge(&mut self, other: Self) {
        *self =
            match (*self, other) {
                (FieldType::TUnicode, FieldType::TUnicode) => FieldType::TUnicode,
                (FieldType::TFloat, FieldType::TFloat) => FieldType::TFloat,
                (FieldType::TInteger, FieldType::TInteger) => FieldType::TInteger,
                (FieldType::TDate, FieldType::TDate) => FieldType::TDate,
                (FieldType::TDateTime, FieldType::TDateTime) => FieldType::TDateTime,
                // Null does not impact the type.
                (FieldType::TNull, any) | (any, FieldType::TNull) => any,
                // There's no way to get around an unknown.
                (FieldType::TUnknown, _) | (_, FieldType::TUnknown) => FieldType::TUnknown,
                // Integers can degrade to floats.
                (FieldType::TFloat, FieldType::TInteger)
                | (FieldType::TInteger, FieldType::TFloat) => FieldType::TFloat,
                // Numbers can degrade to Unicode strings.
                (FieldType::TUnicode, FieldType::TFloat)
                | (FieldType::TFloat, FieldType::TUnicode) => FieldType::TUnicode,
                (FieldType::TUnicode, FieldType::TInteger)
                | (FieldType::TInteger, FieldType::TUnicode) => FieldType::TUnicode,
                // Dates can degrade to Unicode strings.
                (FieldType::TUnicode, FieldType::TDate)
                | (FieldType::TDate, FieldType::TUnicode) => FieldType::TUnicode,
                // DateTimes can degrade to Unicode strings
                (FieldType::TUnicode, FieldType::TDateTime)
                | (FieldType::TDateTime, FieldType::TUnicode) => FieldType::TUnicode,
                // Dates and numbers degrade to Unicode strings
                (FieldType::TDate, FieldType::TInteger)
                | (FieldType::TDate, FieldType::TFloat)
                | (FieldType::TInteger, FieldType::TDate)
                | (FieldType::TFloat, FieldType::TDate) => FieldType::TUnicode,
                (FieldType::TDateTime, FieldType::TInteger)
                | (FieldType::TDateTime, FieldType::TFloat)
                | (FieldType::TInteger, FieldType::TDateTime)
                | (FieldType::TFloat, FieldType::TDateTime) => FieldType::TUnicode,
                // DateTime can degrade to Date.
                (FieldType::TDateTime, FieldType::TDate)
                | (FieldType::TDate, FieldType::TDateTime) => FieldType::TDate,
            };
    }

    pub fn from_sample(sample: &[u8]) -> Self {
        if sample.is_empty() {
            return FieldType::TNull;
        }
        let string = match std::str::from_utf8(sample) {
            Err(_) => return FieldType::TUnknown,
            Ok(s) => s,
        };
        if string.parse::<i64>().is_ok() {
            return FieldType::TInteger;
        }
        if string.parse::<f64>().is_ok() {
            return FieldType::TFloat;
        }
        if string.parse::<DateTime<Utc>>().is_ok() {
            return FieldType::TDateTime;
        }
        if string.parse::<DateTime<FixedOffset>>().is_ok() {
            return FieldType::TDateTime;
        }
        if string.parse::<NaiveDateTime>().is_ok() {
            return FieldType::TDateTime;
        }
        if string.parse::<NaiveDate>().is_ok() {
            return FieldType::TDate;
        }
        // look for %m/%d/%y format. i.e. 07/08/01
        if NaiveDate::parse_from_str(string, "%D").is_ok() {
            return FieldType::TDate;
        }
        // look for %m/%d/%Y format. i.e. 07/08/2019
        if NaiveDate::parse_from_str(string, "%m/%d/%Y").is_ok() {
            return FieldType::TDate;
        }
        // look for %v format. i.e. 8-Jul-2001
        if NaiveDate::parse_from_str(string, "%v").is_ok() {
            return FieldType::TDate;
        }
        FieldType::TUnicode
    }

    pub fn is_number(self) -> bool {
        self == FieldType::TFloat || self == FieldType::TInteger
    }

    pub fn is_null(self) -> bool {
        self == FieldType::TNull
    }
}

impl Default for FieldType {
    // The default is the most specific type.
    // Type inference proceeds by assuming the most specific type and then
    // relaxing the type as counter-examples are found.
    fn default() -> Self {
        FieldType::TNull
    }
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
        panic!();
    }

    #[test]
    fn test_fieldtype_from_sample() {
        let example_unicode = "abc123!";
        let example_int = "123";
        let example_float = "1.23";
        let example_date = "2019-07-10";
        let example_date2 = "7/10/19";
        let example_date3 = "7/10/2019";
        let example_date4 = "8-Jul-2019";
        let example_datetime = "2019-07-10T16:39:57-08:00";
        let example_datetime2 = "2019-07-10T16:39:57Z";

        let sample_unicode = FieldType::from_sample(example_unicode.as_bytes());
        let sample_int = FieldType::from_sample(example_int.as_bytes());
        let sample_float = FieldType::from_sample(example_float.as_bytes());
        let sample_date = FieldType::from_sample(example_date.as_bytes());
        let sample_date2 = FieldType::from_sample(example_date2.as_bytes());
        let sample_date3 = FieldType::from_sample(example_date3.as_bytes());
        let sample_date4 = FieldType::from_sample(example_date4.as_bytes());
        let sample_datetime = FieldType::from_sample(example_datetime.as_bytes());
        let sample_datetime2 = FieldType::from_sample(example_datetime2.as_bytes());
        assert_eq!(FieldType::TUnicode, sample_unicode);
        assert_eq!(FieldType::TInteger, sample_int);
        assert_eq!(FieldType::TFloat, sample_float);
        assert_eq!(FieldType::TDate, sample_date);
        assert_eq!(FieldType::TDate, sample_date2);
        assert_eq!(FieldType::TDate, sample_date3);
        assert_eq!(FieldType::TDate, sample_date4);
        assert_eq!(FieldType::TDateTime, sample_datetime);
        assert_eq!(FieldType::TDateTime, sample_datetime2);
    }

    #[test]
    fn test_check_fieldtype() {
        panic!();
    }

    #[test]
    fn test_schema_from_hashmap() {
        panic!();
    }

    #[test]
    fn test_datasetschema_from_hashmap() {
        panic!();
    }
}
