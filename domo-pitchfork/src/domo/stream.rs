//! Domo Stream API
//!
//! # [`StreamsRequestBuilder`](`crate::pitchfork::StreamsRequestBuilder`) implements all available stream API endpoints and functionality
//!
//! Additional Resources:
//! - [Domo's Stream API Reference](https://developer.domo.com/docs/streams-api-reference/streams)
//!
use crate::domo::dataset::Dataset;
use crate::domo::dataset::DatasetSchema;
use serde::{Deserialize, Serialize};

pub enum UpdateMethod {
    Replace,
    Append,
}
pub enum StreamSearchQuery {
    DatasetId(String),
    DatasetOwnerId(usize),
}

/// Request Builder for Stream API Endpoints
// impl<'t> StreamsRequestBuilder<'t, StreamDataset> {
//     /// Retrieve details for a given Domo Stream
//     ///
//     /// # Example
//     /// ```no_run
//     /// # use domo_pitchfork::error::PitchforkError;
//     /// use domo_pitchfork::pitchfork::DomoPitchfork;
//     /// let domo = DomoPitchfork::with_token("token");
//     /// let stream_id = 123; // stream id to get details for.
//     /// let stream_info = domo.streams().info(stream_id)?;
//     /// println!("Stream Details: \n{:#?}", stream_info);
//     /// # Ok::<(), PitchforkError>(())
//     /// ```
//     pub fn info(mut self, stream_id: u64) -> Result<StreamDataset, PitchforkError> {
//         // TODO: there's an optional fields query param now
//         self.url.push_str(&stream_id.to_string());
//         let req = Self {
//             method: Method::GET,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         };
//         req.retrieve_and_deserialize_json()
//     }


//     /// Returns a list of [`StreamDataset`]s that meet the search query criteria.
//     ///
//     /// # Example
//     /// ```no_run
//     /// # use domo_pitchfork::error::PitchforkError;
//     /// # use domo_pitchfork::domo::stream::StreamSearchQuery;
//     /// use domo_pitchfork::pitchfork::DomoPitchfork;
//     /// let domo = DomoPitchfork::with_token("token");
//     /// let user_id = 123; // User Id to search for streams for.
//     /// let query = StreamSearchQuery::DatasetOwnerId(user_id);
//     /// let list = domo.streams().search(query)?;
//     /// list.iter().map(|s| println!("Dataset Name: {}", s.dataset.name.as_ref().unwrap()));
//     /// # Ok::<(),PitchforkError>(())
//     /// ```
//     pub fn search(
//         mut self,
//         query: StreamSearchQuery,
//     ) -> Result<Vec<StreamDataset>, PitchforkError> {
//         // TODO: optional fields query param
//         let q = match query {
//             StreamSearchQuery::DatasetId(s) => format!("dataSource.id:{}", s),
//             StreamSearchQuery::DatasetOwnerId(user_id) => {
//                 format!("dataSource.owner.id:{}", user_id)
//             }
//         };
//         self.url.push_str(&format!("/search?q={}&fields=all", q));
//         let req = Self {
//             method: Method::GET,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         };
//         let res = req.send_json()?;
//         let ds_list = serde_json::from_reader(res)?;
//         Ok(ds_list)
//     }

//     /// Create a new `StreamDataset` to create executions and upload data to.
//     pub fn create(self, ds_meta: &StreamDatasetSchema) -> Result<StreamDataset, PitchforkError> {
//         let body = serde_json::to_string(ds_meta)?;
//         debug!("body: {}", body);
//         let req = Self {
//             method: Method::POST,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: Some(body),
//         };
//         let res = req.send_json()?;
//         let ds = serde_json::from_reader(res)?;
//         Ok(ds)
//     }

//     /// Delete a given Domo Stream.
//     /// Warning: this action is destructive and cannot be reversed.
//     ///
//     /// # Example
//     /// ```no_run
//     /// # use domo_pitchfork::pitchfork::DomoPitchfork;
//     /// # let token = "token_here";
//     /// let domo = DomoPitchfork::with_token(&token);
//     ///
//     /// let stream_id = 123; //id of stream to delete.
//     /// // if it fails to delete print err msg.
//     /// if let Err(e) = domo.streams().delete(stream_id) {
//     ///     println!("{}", e)
//     /// }
//     /// ```
//     pub fn delete(mut self, stream_id: u64) -> Result<(), PitchforkError> {
//         self.url.push_str(&stream_id.to_string());
//         let req = Self {
//             method: Method::DELETE,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         };
//         req.send_json()?;
//         Ok(())
//     }

//     /// Updates Stream Update Method settings
//     pub fn modify_update_method(
//         mut self,
//         stream_id: u64,
//         update_method: &UpdateMethod,
//     ) -> Result<Dataset, PitchforkError> {
//         self.url.push_str(&stream_id.to_string());
//         let um = match update_method {
//             UpdateMethod::Append => "APPEND",
//             UpdateMethod::Replace => "REPLACE",
//         };
//         let body = json!({ "updateMethod": um }).to_string();
//         debug!("body: {}", body);
//         let req = Self {
//             method: Method::PATCH,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: Some(body),
//         };
//         let ds = serde_json::from_reader(req.send_json()?)?;
//         Ok(ds)
//     }


//     /// Details for a `StreamExecution` for a given `StreamDataset`
//     ///
//     /// # Example
//     /// ```no_run
//     /// # use domo_pitchfork::error::PitchforkError;
//     /// use domo_pitchfork::pitchfork::DomoPitchfork;
//     /// let domo = DomoPitchfork::with_token("token");
//     /// let stream_id = 123; // stream id to get execution info for.
//     /// let ex_id = 1; // execution id to get info for.
//     /// let execution_info = domo.streams().execution_info(stream_id, ex_id)?;
//     /// println!("Stream Execution Details: \n{:#?}", execution_info);
//     /// # Ok::<(), PitchforkError>(())
//     /// ```
//     pub fn execution_info(
//         mut self,
//         stream_id: u64,
//         execution_id: u32,
//     ) -> Result<StreamExecution, PitchforkError> {
//         self.url
//             .push_str(&format!("{}/executions/{}", stream_id, execution_id));
//         let req = Self {
//             method: Method::GET,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         };
//         let se = serde_json::from_reader(req.send_json()?)?;
//         Ok(se)
//     }

//     /// List Domo Executions for a given Domo Stream.
//     /// Max limit is 500.
//     /// Offset is the offset of the Stream ID to begin list of streams within the response
//     ///
//     /// # Example
//     /// ```no_run
//     /// # use domo_pitchfork::error::PitchforkError;
//     /// use domo_pitchfork::pitchfork::DomoPitchfork;
//     /// let domo = DomoPitchfork::with_token("token");
//     /// let stream_id = 123; // stream id to retrieve executions for.
//     /// let list = domo.streams().list_executions(stream_id, 50,0)?;
//     /// list.iter().map(|s| println!("Execution Id: {}", s.id));
//     /// # Ok::<(),PitchforkError>(())
//     /// ```
//     pub fn list_executions(
//         mut self,
//         stream_id: u64,
//         limit: u32,
//         offset: u32,
//     ) -> Result<Vec<StreamExecution>, PitchforkError> {
//         self.url.push_str(&format!(
//             "{}/executions?limit={}&offset={}",
//             stream_id, limit, offset
//         ));
//         let req = Self {
//             method: Method::GET,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         };
//         let res = req.send_json()?;
//         let ds_list = serde_json::from_reader(res)?;
//         Ok(ds_list)
//     }
//     /// Upload a data part to a stream execution in progress.
//     /// Parts can be uploaded simultaneously and in any order.
//     pub fn upload_part(
//         mut self,
//         stream_id: u64,
//         execution_id: u32,
//         part: u32,
//         csv_part: &str,
//     ) -> Result<StreamExecution, PitchforkError> {
//         self.url.push_str(&format!(
//             "{}/executions/{}/part/{}",
//             stream_id, execution_id, part
//         ));
//         let req = Self {
//             method: Method::PUT,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: Some(csv_part.to_string()),
//         };
//         let ds_list = serde_json::from_reader(req.send_csv()?)?;
//         Ok(ds_list)
//     }

// [Stream Object](https://developer.domo.com/docs/streams-api-reference/streams
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamDataset {
    pub id: u64,
    #[serde(rename = "dataSet")]
    pub dataset: Dataset,
    #[serde(rename = "updateMethod")]
    pub update_method: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "lastExecution")]
    pub last_execution: Option<StreamExecution>,
}

// [Stream Object](https://developer.domo.com/docs/streams-api-reference/streams
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomoStream {
    pub id: usize,
    #[serde(rename = "dataSet")]
    pub dataset: Dataset,
    #[serde(rename = "updateMethod")]
    pub update_method: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "lastExecution")]
    pub last_execution: Option<StreamExecution>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamExecution {
    pub id: usize,
    #[serde(rename = "startedAt")]
    pub started_at: Option<String>,
    #[serde(rename = "endedAt")]
    pub ended_at: Option<String>,
    #[serde(rename = "currentState")]
    pub current_state: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamDatasetSchema {
    #[serde(rename = "dataSet")]
    pub dataset_schema: DatasetSchema,
    #[serde(rename = "updateMethod")]
    pub update_method: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewStreamDataset {
    #[serde(rename = "dataSet")]
    pub dataset: StreamDatasetSchema,
}

//TODO: impl UpdateMethod

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json;
//     #[test]
//     fn test_dataset_schema_serialization() {
//         let c = Column {
//             column_type: "STRING".to_string(),
//             name: "column name".to_string(),
//         };
//         let s = Schema { columns: vec![c] };
//         let d_schema = DatasetSchema {
//             name: "test dataset".to_string(),
//             description: "test description".to_string(),
//             rows: 0,
//             schema: s,
//         };
//         let expected = json!({
// 								"name": "test dataset",
// 								"description": "test description",
// 								"rows": 0,
// 								"schema": {
// 									"columns": [{
// 										"type": "STRING",
// 										"name": "column name"
// 									}]
// 								},
// 							});

//         let v = serde_json::to_value(d_schema).unwrap();
//         assert_eq!(v, expected);
//     }

//     #[test]
//     fn test_fieldtype_merge() {
//         assert!(false);
//     }

//     #[test]
//     fn test_fieldtype_from_sample() {
//         assert!(false);
//     }

//     #[test]
//     fn test_check_fieldtype() {
//         assert!(false);
//     }

//     #[test]
//     fn test_schema_from_hashmap() {
//         assert!(false);
//     }

//     #[test]
//     fn test_datasetschema_from_hashmap() {
//         assert!(false);
//     }

// }
