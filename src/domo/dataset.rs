//! Domo Dataset API
//!
//! # [`DatasetsRequestBuilder`](`crate::pitchfork::DatasetsRequestBuilder`) implements all available dataset API endpoints and functionality
//!
//! Additional Resources:
//! - [Domo Dataset API Reference](https://developer.domo.com/docs/dataset-api-reference/dataset)
use super::policy::Policy;
use super::user::Owner;
use crate::util::csv::serialize_to_csv_str;
use serde_json::json;
use serde_json::Value;

use crate::error::DomoError;
use crate::pitchfork::{DatasetsRequestBuilder, DomoRequest};
use log::debug;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::marker::PhantomData;
use std::str::{self, FromStr};

impl<'t> DatasetsRequestBuilder<'t, Dataset> {
    /// Retreives details for a `Dataset`
    ///
    /// # Example
    /// ```no_run
    /// # use domo_pitchfork::error::DomoError;
    /// use domo_pitchfork::pitchfork::DomoPitchfork;
    /// let domo = DomoPitchfork::with_token("token");
    /// let dataset_info = domo.datasets().info("dataset id")?;
    /// println!("Dataset Details: \n{:#?}", dataset_info);
    /// # Ok::<(), DomoError>(())
    /// ```
    ///
    pub fn info(mut self, dataset_id: &str) -> Result<Dataset, DomoError> {
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
    /// # use domo_pitchfork::error::DomoError;
    /// use domo_pitchfork::pitchfork::DomoPitchfork;
    /// let domo = DomoPitchfork::with_token("token");
    /// let dataset_list = domo.datasets().list(5,0)?;
    /// dataset_list.iter().map(|ds| println!("Dataset Name: {}", ds.name.as_ref().unwrap()));
    /// # Ok::<(),DomoError>(())
    /// ```
    pub fn list(mut self, limit: u32, offset: u32) -> Result<Vec<Dataset>, DomoError> {
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
    pub fn create(self, ds_meta: &DatasetSchema) -> Result<Dataset, DomoError> {
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
    pub fn delete(mut self, dataset_id: &str) -> Result<(), DomoError> {
        self.url.push_str(dataset_id);
        let req = Self {
            method: Method::DELETE,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        let res = req.send_json()?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(DomoError::Other(format!("HTTP Status: {}", res.status())))
        }
    }

    /// Modify an existing Domo Dataset.
    pub fn modify(
        mut self,
        dataset_id: &str,
        ds_meta: &DatasetSchema,
    ) -> Result<Dataset, DomoError> {
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
    ) -> Result<DatasetQueryData, DomoError> {
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

    /// Retrieve data from a Domo Dataset.
    pub fn download_data(
        mut self,
        dataset_id: &str,
        include_csv_headers: bool,
    ) -> Result<String, DomoError> {
        self.url.push_str(&format!(
            "{}/data?includeHeader={}",
            dataset_id, include_csv_headers
        ));
        self.send_json()?.text().map_err(|e| e.into())
    }

    /// Upload data to the Domo Dataset.
    pub fn upload_from_str(mut self, dataset_id: &str, data_rows: String) -> Result<(), DomoError> {
        self.url.push_str(&format!("{}/data", dataset_id));
        let req = Self {
            method: Method::PUT,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(data_rows),
        };
        let mut res = req.send_csv()?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(DomoError::Other(format!(
                "HTTP Status: {}\nMessage: {}",
                res.status(),
                res.text().unwrap_or_else(|_| String::new())
            )))
        }
    }

    /// Upload data to the Domo Dataset.
    pub fn upload_serializable<T: Serialize>(
        mut self,
        dataset_id: &str,
        data: &[T],
    ) -> Result<(), DomoError> {
        self.url.push_str(&format!("{}/data", dataset_id));
        let req = Self {
            method: Method::PUT,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(serialize_to_csv_str(&data)?),
        };
        let mut res = req.send_csv()?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(DomoError::Other(format!(
                "HTTP Status: {}\nMessage: {}",
                res.status(),
                res.text().unwrap_or_else(|_| String::new())
            )))
        }
    }

    /// Retrieves details of a given policy for a Dataset
    pub fn pdp_policy_info(
        mut self,
        dataset_id: &str,
        policy_id: u32,
    ) -> Result<Policy, DomoError> {
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
    ) -> Result<Policy, DomoError> {
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
    ) -> Result<Policy, DomoError> {
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
    pub fn delete_pdp_policy(mut self, dataset_id: &str, policy_id: u32) -> Result<(), DomoError> {
        self.url
            .push_str(&format!("{}/policies/{}", dataset_id, policy_id));
        let req = Self {
            method: Method::DELETE,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        let mut res = req.send_json()?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(DomoError::Other(format!(
                "HTTP Status: {}\nMessage: {}",
                res.status(),
                res.text().unwrap_or_else(|_| String::new())
            )))
        }
    }

    /// Retrieves a list of all policies for a Dataset
    pub fn policies(mut self, dataset_id: &str) -> Result<Vec<Policy>, DomoError> {
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
            let typ_str = match typ {
                FieldType::TUnicode => "STRING".to_string(),
                FieldType::TFloat => "DOUBLE".to_string(),
                FieldType::TInteger => "LONG".to_string(),
                _ => "STRING".to_string(),
            };
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
    // TODO: document where this is needed
    #[allow(dead_code)]
    fn from_fieldtype(typ: FieldType) -> Self {
        match typ {
            FieldType::TNull | FieldType::TUnicode => DomoDataType::STRING,
            // TUnicode => DomoDataType::STRING,
            FieldType::TInteger => DomoDataType::LONG,
            FieldType::TFloat => DomoDataType::DECIMAL,
            _ => DomoDataType::STRING,
        }
    }
}

impl From<DomoDataType> for String {
    fn from(domo_type: DomoDataType) -> Self {
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
}

impl FieldType {
    pub fn merge(&mut self, other: Self) {
        *self =
            match (*self, other) {
                (FieldType::TUnicode, FieldType::TUnicode) => FieldType::TUnicode,
                (FieldType::TFloat, FieldType::TFloat) => FieldType::TFloat,
                (FieldType::TInteger, FieldType::TInteger) => FieldType::TInteger,
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
            };
    }

    pub fn from_sample(sample: &[u8]) -> Self {
        if sample.is_empty() {
            return FieldType::TNull;
        }
        let string = match str::from_utf8(sample) {
            Err(_) => return FieldType::TUnknown,
            Ok(s) => s,
        };
        if string.parse::<i64>().is_ok() {
            return FieldType::TInteger;
        }
        if string.parse::<f64>().is_ok() {
            return FieldType::TFloat;
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
        panic!();
    }

    #[test]
    fn test_fieldtype_from_sample() {
        panic!();
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
