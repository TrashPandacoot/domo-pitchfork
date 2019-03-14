//! Client for Domo API
use lazy_static::lazy_static;
use reqwest::Body;
use reqwest::Client;
use reqwest::Method;
use serde::de::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::json;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::string::String;

use crate::auth::DomoClientAppCredentials;
use crate::domo::activity_log::ActivityLog;
use crate::domo::dataset::{Dataset, DatasetSchema};
use crate::domo::group::GroupInfo;
use crate::domo::page::{PageCollection, PageInfo};
use crate::domo::stream::{StreamDataset, StreamDatasetSchema, StreamExecution};
use crate::domo::user::User;
use crate::error::DomoError;
use crate::util::common::convert_map_to_string;
use crate::util::common::{gzip_csv, gzip_str};
use crate::util::csv::serialize_to_csv_str;

lazy_static! {
    /// Static HTTP Client for Domo API
    pub static ref CLIENT: Client = Client::new();
}

/// Domo API Client
pub struct RustyPitchfork {
    /// Base URI for Domo API. Usually set implicitly with default() constructor.
    pub base_uri: String,
    /// Domo Access Token. Set by the auth_manager.
    pub access_token: Option<String>,
    /// Authentication Manager used to authenticate client and
    /// obtain access_token.
    pub auth_manager: Option<DomoClientAppCredentials>,
}




impl PitchFork for RustyPitchfork {
    /// Create Authorization HTTP header.
    fn auth_headers(&self) -> String {
        match self.access_token {
            Some(ref token) => token.to_string(),
            None => match self.auth_manager {
                Some(ref auth_manager) => auth_manager.get_access_token(),
                None => panic!("Auth manager hasn't been set"),
            },
        }
    }
}

impl RustyPitchfork {
    pub fn default() -> Self {
        Self {
            base_uri: "https://api.domo.com/".to_string(),
            access_token: None,
            auth_manager: None,
        }
    }

    /// Set the base uri for the Domo API HTTP requests.
    pub fn base_uri(mut self, base_uri: &str) -> Self {
        self.base_uri = base_uri.to_string();
        self
    }

    // TODO: Does this need to be public?
    /// Set the access token
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_string());
        self
    }

    /// Create the auth manager with given Domo Client Id and Client Secret
    pub fn auth_manager(mut self, auth_manager: DomoClientAppCredentials) -> Self {
        self.auth_manager = Some(auth_manager);
        self
    }

    /// Create RustyPitchfork Instance
    pub fn build(self) -> Self {
        if self.access_token.is_none() && self.auth_manager.is_none() {
            panic!("Token and Auth manager are None");
        }
        self
    }

    /// Custom Domo Query.
    pub fn domo_get(
        &self,
        url: &str,
        params: &HashMap<String, String>,
    ) -> Result<String, DomoError> {
        self.get(url, params)
    }

    // Start Dataset APIs

    ///[get-dataset](https://developer.domo.com/docs/dataset-api-reference/dataset)
    /// returns a single `Dataset` details given the dataset id
    /// Parameters:
    /// - dataset_id - a domo dataset Id
    pub fn dataset(&self, dataset_id: &str) -> Result<Dataset, DomoError> {
        let url = format!("v1/datasets/{}", dataset_id);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Dataset>(&result.unwrap_or_default())
    }

    pub fn create_dataset(&self, dataset: DatasetSchema) -> Result<Dataset, DomoError> {
        let url = "v1/datasets".to_string();
        let val = serde_json::to_value(dataset).unwrap();
        let result = self.post(&url, &val);
        self.convert_result::<Dataset>(&result.unwrap_or_default())
    }

    pub fn list_datasets(&self, limit: i32, offset: i32) -> Result<Vec<Dataset>, DomoError> {
        let url = format!("v1/datasets?limit={0}&offset={1}", limit, offset);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<Dataset>>(&result.unwrap_or_default())
    }

    pub fn update_dataset_meta(
        &self,
        dataset_id: &str,
        dataset: DatasetSchema,
    ) -> Result<Dataset, DomoError> {
        let url = format!("v1/datasets/{}", dataset_id);
        let val = serde_json::to_value(dataset).unwrap();
        let result = self.put(&url, &val);
        self.convert_result::<Dataset>(&result.unwrap_or_default())
    }

    pub fn delete_dataset(&self, dataset_id: &str) -> Result<String, DomoError> {
        let url = format!("v1/datasets/{}", dataset_id);
        let result = self.delete(&url, &json!({}))?;
        Ok(result)
    }

    pub fn replace_data(&self, dataset_id: &str, data_rows: &str) -> Result<String, DomoError> {
        let url = format!("v1/datasets/{}/data", dataset_id);
        let result = self
            .post_csv(Method::PUT, &url, &data_rows)
            .unwrap_or_default();
        Ok(result)
    }

    pub fn replace_data_with_vec<T: Serialize>(
        &self,
        dataset_id: &str,
        data: &[T],
    ) -> Result<String, DomoError> {
        let data_rows: String = serialize_to_csv_str(&data)?;
        let url = format!("v1/datasets/{}/data", dataset_id);
        let result = self
            .post_csv(Method::PUT, &url, &data_rows)
            .unwrap_or_default();
        Ok(result)
    }

    pub fn replace_data_with_file(
        &self,
        dataset_id: &str,
        file_path: &str,
    ) -> Result<String, DomoError> {
        let url = format!("v1/datasets/{}/data", dataset_id);
        let result = self
            .post_csv_file(Method::PUT, &url, &file_path)
            .unwrap_or_default();
        Ok(result)
    }

    pub fn export_data(&self, dataset_id: &str) -> Result<String, DomoError> {
        let url = format!("v1/datasets/{}/data?includeHeader=true", dataset_id);
        let result = self.get(&url, &HashMap::new())?;
        Ok(result)
    }
    // End of Dataset APIs

    // Stream_api start
    /// Create a new `StreamDataset` to create executions and upload data to.
    pub fn create_stream(&self, schema: StreamDatasetSchema) -> Result<StreamDataset, DomoError> {
        let url = "v1/streams".to_string();
        let val = serde_json::to_value(schema).unwrap();
        let result = self.post(&url, &val)?;
        self.convert_result::<StreamDataset>(&result)
    }

    pub fn stream_details(&self, stream_id: i32) -> Result<StreamDataset, DomoError> {
        let url = format!("v1/streams/{}?fields=all", stream_id);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<StreamDataset>(&result.unwrap_or_default())
    }

    pub fn list_streams(&self, limit: i32, offset: i32) -> Result<Vec<StreamDataset>, DomoError> {
        let url = format!("v1/streams?limit={0}&offset={1}", limit, offset);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<StreamDataset>>(&result.unwrap_or_default())
    }

    // TODO: remove this if it's one of the methods that was in Domo's docs but "shouldn't have been"
    pub fn list_streams_by_owner(&self, owner_id: i32) -> Result<Vec<StreamDataset>, DomoError> {
        let url = format!(
            "v1/streams/search?q=dataSource.owner.id:{0}&fields=all",
            owner_id
        );
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<StreamDataset>>(&result.unwrap_or_default())
    }

    // TODO: remove this if it's one of the methods that was in Domo's docs but "shouldn't have been"
    pub fn search_stream_by_dataset_id(
        &self,
        dataset_id: &str,
    ) -> Result<Vec<StreamDataset>, DomoError> {
        let url = format!(
            "v1/streams/search?q=dataSource.id:{0}&fields=all",
            dataset_id
        );
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<StreamDataset>>(&result.unwrap_or_default())
    }

    /// Updates Stream UploadMethod settings
    pub fn update_stream_meta(
        &self,
        stream_id: i32,
        stream_dataset: StreamDataset,
    ) -> Result<StreamDataset, DomoError> {
        let url = format!("v1/streams/{}", stream_id);
        let val = serde_json::to_value(stream_dataset).unwrap();
        let result = self.put(&url, &val);
        self.convert_result::<StreamDataset>(&result.unwrap_or_default())
    }

    pub fn delete_stream(&self, stream_id: i32) -> Result<(), DomoError> {
        let url = format!("v1/streams/{}", stream_id);
        let _ = self.delete(&url, &json!({}))?;
        Ok(())
    }

    /// Create a `StreamExecution` to upload data parts to and update the data in Domo.
    pub fn create_stream_execution(&self, stream_id: i32) -> Result<StreamExecution, DomoError> {
        let url = format!("v1/streams/{}/executions", stream_id);
        let result = self.post(&url, &json!({}));
        self.convert_result::<StreamExecution>(&result.unwrap_or_default())
    }

    pub fn list_stream_executions(
        &self,
        stream_id: i64,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<StreamExecution>, DomoError> {
        let url = format!(
            "v1/streams/{0}/executions?limit={1}&offset={2}",
            stream_id, limit, offset
        );
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<StreamExecution>>(&result.unwrap_or_default())
    }

    /// Upload a data part to a stream execution in progress
    pub fn upload_data_part(
        &self,
        stream_id: i32,
        execution_id: i32,
        part: i32,
        csv_part: &str,
    ) -> Result<(), DomoError> {
        let url = format!(
            "v1/streams/{0}/executions/{1}/part/{2}",
            stream_id, execution_id, part
        );
        let gzipped_data = gzip_str(&csv_part);
        let _ = self.post_compressed_csv(Method::PUT, &url, gzipped_data)?;
        Ok(())
    }

    /// Upload a data part to a stream execution in progress where the data part
    /// is a `Serializable` vec of T.
    pub fn upload_data_part_with_vec<T: Serialize>(
        &self,
        stream_id: i32,
        execution_id: i32,
        part: i32,
        data: &[T],
    ) -> Result<(), DomoError> {
        let url = format!(
            "v1/streams/{0}/executions/{1}/part/{2}",
            stream_id, execution_id, part
        );
        let csv_part = serialize_to_csv_str(data)?;
        let gzipped_data = gzip_str(&csv_part);
        let _ = self.post_compressed_csv(Method::PUT, &url, gzipped_data)?;
        Ok(())
    }

    /// Upload a csv file as a data part to a stream execution in progress.
    pub fn upload_data_part_file(
        &self,
        stream_id: i32,
        execution_id: i32,
        part: i32,
        csv_path: &str,
    ) -> Result<(), DomoError> {
        let url = format!(
            "v1/streams/{0}/executions/{1}/part/{2}",
            stream_id, execution_id, part
        );
        //let csv_part = util::gzip_csv(csv_path);
        let csv_part = gzip_csv(csv_path).unwrap();
        let _ = self.post_compressed_csv(Method::PUT, &url, csv_part)?;
        Ok(())
    }

    /// Commit a stream execution and finalize insertion of dataparts into Domo Stream Dataset.
    pub fn commit_execution(
        &self,
        stream_id: i32,
        execution_id: i32,
    ) -> Result<StreamExecution, DomoError> {
        let url = format!(
            "v1/streams/{0}/executions/{1}/commit",
            stream_id, execution_id
        );
        let result = self.put(&url, &json!({}));
        self.convert_result::<StreamExecution>(&result.unwrap_or_default())
    }

    /// Abort a stream execution in progress and discard all data parts uploaded to the execution.
    pub fn abort_stream_execution(
        &self,
        stream_id: i32,
        execution_id: i32,
    ) -> Result<(), DomoError> {
        let url = format!(
            "v1/streams/{0}/executions/{1}/abort",
            stream_id, execution_id
        );
        let _ = self.put(&url, &json!({}))?;
        Ok(())
    }
    // Stream_api end
    // Start of Group APIs
    pub fn group(&self, group_id: u64) -> Result<GroupInfo, DomoError> {
        let url = format!("v1/groups/{0}", group_id);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<GroupInfo>(&result.unwrap_or_default())
    }

    pub fn list_groups(&self, limit: u32, offset: u32) -> Result<Vec<GroupInfo>, DomoError> {
        let url = format!("v1/groups?limit={0}&offset={1}", limit, offset);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<GroupInfo>>(&result.unwrap_or_default())
    }

    pub fn create_group(&self, group: GroupInfo) -> Result<GroupInfo, DomoError> {
        let url = "v1/groups".to_string();
        let val = serde_json::to_value(group).unwrap();
        let result = self.post(&url, &val);
        self.convert_result::<GroupInfo>(&result.unwrap_or_default())
    }

    pub fn update_group(&self, group_id: u64, group: GroupInfo) -> Result<(), DomoError> {
        let url = format!("v1/groups/{0}", group_id);
        let val = serde_json::to_value(group).unwrap();
        let _ = self.put(&url, &val)?;
        Ok(())
    }

    pub fn delete_group(&self, group_id: u64) -> Result<(), DomoError> {
        let url = format!("v1/groups/{0}", group_id);
        let _ = self.delete(&url, &json!({}))?;
        Ok(())
    }

    pub fn list_group_users(&self, group_id: u64) -> Result<Vec<u64>, DomoError> {
        let url = format!("v1/groups/{0}/users", group_id);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<u64>>(&result.unwrap_or_default())
    }

    pub fn add_group_user(&self, group_id: u64, user_id: u64) -> Result<(), DomoError> {
        let url = format!("v1/groups/{0}/users/{1}", group_id, user_id);
        let _ = self.put(&url, &json!({}))?;
        Ok(())
    }

    pub fn remove_group_user(&self, group_id: u64, user_id: u64) -> Result<(), DomoError> {
        let url = format!("v1/groups/{0}/users/{1}", group_id, user_id);
        let _ = self.delete(&url, &json!({}))?;
        Ok(())
    }
    // End of Groups APIs
    // Start of User APIs
    pub fn user(&self, user_id: u64) -> Result<User, DomoError> {
        let url = format!("v1/users/{0}", user_id);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<User>(&result.unwrap_or_default())
    }

    pub fn list_users(&self, limit: u32, offset: u32) -> Result<Vec<User>, DomoError> {
        let url = format!("v1/users?limit={0}&offset={1}", limit, offset);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<User>>(&result.unwrap_or_default())
    }

    pub fn create_user(&self, user: User) -> Result<User, DomoError> {
        let url = "v1/users".to_string();
        let val = serde_json::to_value(user).unwrap();
        let result = self.post(&url, &val);
        self.convert_result::<User>(&result.unwrap_or_default())
    }

    pub fn update_user(&self, user_id: u64, user: User) -> Result<(), DomoError> {
        let url = format!("v1/users/{0}", user_id);
        let val = serde_json::to_value(user).unwrap();
        let _ = self.put(&url, &val)?;
        Ok(())
    }

    pub fn delete_user(&self, user_id: u64) -> Result<(), DomoError> {
        let url = format!("v1/users/{0}", user_id);
        let _ = self.delete(&url, &json!({}))?;
        Ok(())
    }

    // End of User APIs
    // Start of Page APIs

    pub fn page(&self, page_id: u64) -> Result<PageInfo, DomoError> {
        let url = format!("v1/pages/{0}", page_id);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<PageInfo>(&result.unwrap_or_default())
    }

    pub fn list_pages(&self, limit: u32, offset: u32) -> Result<Vec<PageInfo>, DomoError> {
        let url = format!("v1/pages?limit={0}&offset={1}", limit, offset);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<PageInfo>>(&result.unwrap_or_default())
    }

    pub fn create_page(&self, page: PageInfo) -> Result<PageInfo, DomoError> {
        let url = "v1/pages".to_string();
        let val = serde_json::to_value(page).unwrap();
        let result = self.post(&url, &val);
        self.convert_result::<PageInfo>(&result.unwrap_or_default())
    }

    pub fn update_page(&self, page_id: u64, page: PageInfo) -> Result<PageInfo, DomoError> {
        let url = format!("v1/pages/{0}", page_id);
        let val = serde_json::to_value(page).unwrap();
        let result = self.put(&url, &val);
        self.convert_result::<PageInfo>(&result.unwrap_or_default())
    }

    pub fn delete_page(&self, page_id: u64) -> Result<(), DomoError> {
        let url = format!("v1/pages/{0}", page_id);
        let _ = self.delete(&url, &json!({}))?;
        Ok(())
    }

    pub fn list_page_collection(&self, page_id: u64) -> Result<Vec<PageCollection>, DomoError> {
        let url = format!("v1/pages/{0}/collections", page_id);
        let result = self.get(&url, &HashMap::new());
        self.convert_result::<Vec<PageCollection>>(&result.unwrap_or_default())
    }

    pub fn create_page_collection(
        &self,
        page_id: u64,
        collection: PageCollection,
    ) -> Result<(), DomoError> {
        let url = format!("v1/pages/{0}/collections", page_id);
        let val = serde_json::to_value(collection).unwrap();
        let _ = self.post(&url, &val)?;
        Ok(())
    }

    pub fn update_page_collection(
        &self,
        page_id: u64,
        collection_id: u64,
        collection: PageCollection,
    ) -> Result<(), DomoError> {
        let url = format!("v1/pages/{0}/collections/{1}", page_id, collection_id);
        let val = serde_json::to_value(collection).unwrap();
        let _ = self.put(&url, &val)?;
        Ok(())
    }

    pub fn delete_page_collection(
        &self,
        page_id: u64,
        collection_id: u64,
    ) -> Result<(), DomoError> {
        let url = format!("v1/pages/{0}/collections/{1}", page_id, collection_id);
        let _ = self.delete(&url, &json!({}))?;
        Ok(())
    }

    // End of Page APIs
    // Start Activity Log Entries APIs
    /// # Activity Log Entries
    /// params can be `user`, `start`, `end`, `limit`, and `offset`
    /// [Activity Log API](https://developer.domo.com/docs/activity-log-api-reference/activity-log)
    pub fn list_activity_log_entries(
        &self,
        params: &HashMap<String, String>,
    ) -> Result<Vec<ActivityLog>, DomoError> {
        let url = "v1/audit".to_string();
        let result = self.get(&url, params);
        self.convert_result::<Vec<ActivityLog>>(&result.unwrap_or_default())
    }
}

pub trait PitchFork {
    fn auth_headers(&self) -> String;

    /// Internal method call to create the appropriate HTTP request.
    fn internal_call(
        &self,
        method: Method,
        url: &str,
        payload: &Value,
    ) -> Result<String, DomoError> {
        let mut url: Cow<'_, str> = url.into();
        if !url.starts_with("http") {
            url = ["https://api.domo.com/", &url].concat().into();
        }

        // let mut headers = Headers::new();
        // headers.set(self.auth_headers());
        // headers.set(ContentType::json());
        let mut response = CLIENT
            .request(method, &url.into_owned())
            // .headers(headers)
            .bearer_auth(self.auth_headers())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .expect("request failed");

        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("failed to read response");
        if response.status().is_success() {
            Ok(buf)
        } else {
            eprintln!("parameters: {:?}\n", &payload);
            eprintln!("response: {:?}", &response);
            eprintln!("content: {:?}", &buf);
            // bail!(
            //     "send request failed, http code:{}, error message:{}",
            //     response.status(),
            //     &buf
            // );
            Err(DomoError::Other("internal call".to_owned()))
        }
    }

    /// POST HTTP request to send a CSV file to the Domo API with.
    fn post_csv_file(
        &self,
        method: Method,
        url: &str,
        input_path: &str,
    ) -> Result<String, DomoError> {
        let mut url: Cow<'_, str> = url.into();
        if !url.starts_with("https") {
            url = ["https://api.domo.com/", &url].concat().into();
        }
        let file = File::open(input_path).expect("Couldn't Open File");
        let body = Body::new(file);
        // let mut headers = Headers::new();
        // headers.set(self.auth_headers());
        // headers.set(ContentType(mime::TEXT_CSV));
        let mut response = CLIENT
            .request(method, &url.into_owned())
            .bearer_auth(self.auth_headers())
            .header("Content-Type", "text/csv")
            // .headers(headers)
            .body(body)
            .send()
            .expect("ಠ_ಠ you just got Domo'd");

        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("ಠ_ಠ failed to read response");
        if response.status().is_success() {
            Ok(buf)
        } else {
            //eprintln!("headers: {:?}", &headers);
            eprintln!("response: {:?}", &response);
            //eprintln!("req body: {}", &string_content);
            // bail!(
            //     "request failed, http code:{}, message:{}",
            //     response.status(),
            //     &buf
            // );
            Err(DomoError::Other("post csv file".to_owned()))
        }
    }

    /// POST HTTP request to send a csv string to the Domo API with.
    fn post_csv(
        &self,
        method: Method,
        url: &str,
        string_content: &str,
    ) -> Result<String, DomoError> {
        let mut url: Cow<'_, str> = url.into();
        if !url.starts_with("https") {
            url = ["https://api.domo.com/", &url].concat().into();
        }
        let string_content = String::from(string_content);
        // let mut headers = Headers::new();
        // headers.set(self.auth_headers());
        // headers.set(ContentType(mime::TEXT_CSV));
        let mut response = CLIENT
            .request(method, &url.into_owned())
            .bearer_auth(self.auth_headers())
            .header("Content-Type", "text/csv")
            // .headers(headers)
            .body(string_content)
            .send()
            .expect("ಠ_ಠ you just got Domo'd");

        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("ಠ_ಠ failed to read response");
        if response.status().is_success() {
            Ok(buf)
        } else {
            //eprintln!("headers: {:?}", &headers);
            eprintln!("response: {:?}", &response);
            //eprintln!("req body: {}", &string_content);
            // bail!(
            //     "request failed, http code:{}, message:{}",
            //     response.status(),
            //     &buf
            // );
            Err(DomoError::Other("post csv".to_owned()))
        }
    }

    /// POST gzipped csv to Domo API.
    fn post_compressed_csv(
        &self,
        method: Method,
        url: &str,
        body_content: Vec<u8>,
    ) -> Result<String, DomoError> {
        let mut url: Cow<'_, str> = url.into();
        if !url.starts_with("https") {
            url = ["https://api.domo.com/", &url].concat().into();
        }
        // let mut headers = Headers::new();
        // headers.set(self.auth_headers());
        // headers.set(ContentType(mime::TEXT_CSV));
        // headers.set(ContentEncoding(vec![Encoding::Gzip]));
        let mut response = CLIENT
            .request(method, &url.into_owned())
            .bearer_auth(self.auth_headers())
            .header("Content-Type", "text/csv")
            .header("Content-Encoding", "gzip")
            // .headers(headers)
            .body(body_content)
            .send()
            .expect("ಠ_ಠ you just got Domo'd");

        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("ಠ_ಠ failed to read response");
        if response.status().is_success() {
            Ok(buf)
        } else {
            //eprintln!("headers: {:?}", &headers);
            eprintln!("response: {:?}", &response);
            //eprintln!("req body: {}", &string_content);
            // bail!(
            //     "request failed, http code:{}, message:{}",
            //     response.status(),
            //     &buf
            // );
            Err(DomoError::Other("post compressed csv".to_owned()))
        }
    }

    ///GET request
    fn get(&self, url: &str, params: &HashMap<String, String>) -> Result<String, DomoError> {
        if params.is_empty() {
            self.internal_call(Method::GET, url, &json!({}))
        } else {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(Method::GET, &url_with_params, &json!({})) 
        }
    }

    ///POST request
    fn post(&self, url: &str, payload: &Value) -> Result<String, DomoError> {
        self.internal_call(Method::POST, url, payload)
    }
    ///PUT request
    fn put(&self, url: &str, payload: &Value) -> Result<String, DomoError> {
        self.internal_call(Method::PUT, url, payload)
    }

    ///DELETE request
    fn delete(&self, url: &str, payload: &Value) -> Result<String, DomoError> {
        self.internal_call(Method::DELETE, url, payload)
    }

    fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Result<T, DomoError> {
        let result = serde_json::from_str::<T>(input)
            // .context(format!("Serde Deserialization failed. content {:?}", input))
            ?;
        Ok(result)
    }
}
