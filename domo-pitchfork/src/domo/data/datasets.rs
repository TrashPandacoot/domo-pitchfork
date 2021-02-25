use std::sync::Arc;

use crate::{DomoApi, domo::dataset::{Dataset, DatasetQueryData}, error::{DomoApiError, DomoErr}, util::csv::serialize_csv_str};
use log::error;
use serde::Serialize;
use serde_json::json;

pub struct DatasetApiBuilder {
    pub(crate) client: Arc<DomoApi>
}

impl DatasetApiBuilder {
    pub fn list(self) -> DatasetApiListBuilder {
        DatasetApiListBuilder::new(self.client)
    }
    pub async fn info(self, dataset_id: &str) -> Result<Dataset, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let token = self.client.auth.get_token().await?;
        let uri = format!("https://api.domo.com/v1/datasets/{}", dataset_id);
        let req = self.client.client.get(&uri)
            .bearer_auth(token)
            .send().await?
            .error_for_status()?;
        let s = req.json().await?;
        Ok(s)
    }
    pub async fn delete(self, dataset_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let token = self.client.auth.get_token().await?;
        let uri = format!("https://api.domo.com/v1/datasets/{}", dataset_id);
        let req = self.client.client.delete(&uri)
            .bearer_auth(token)
            .send().await?
            .error_for_status()?;
        Ok(())
    }
    pub fn query_data(self, dataset_id: &str, sql_query: &str) -> DatasetApiQueryDataBuilder {
        DatasetApiQueryDataBuilder::new(self.client, dataset_id, sql_query)
    }
    pub fn get_data(self, dataset_id: &str) -> DatasetApiGetDataBuilder {
        DatasetApiGetDataBuilder::new(self.client, dataset_id)
    }
    pub fn upload(self, dataset_id: &str) -> DatasetApiUploadBuilder {
        DatasetApiUploadBuilder::new(self.client, dataset_id)
    }
    // pub fn create()
    // pub fn modify()
    // pub fn pdp()
    // pdp_policy_info
    // add pdp policy
    // modify pdp policy
    // delete pdp policy
    // list pdp policies
}
pub struct DatasetApiUploadBuilder {
    api: Arc<DomoApi>,
    dataset_id: String,
    data: Option<String>,
}
impl DatasetApiUploadBuilder {
    pub fn new(client: Arc<DomoApi>, dataset_id: &str) -> Self {
        Self {
            api: client,
            dataset_id: dataset_id.to_string(),
            data: None,
        }
    }

    pub fn data<T: Serialize>(&mut self, data: &[T]) -> Result<&mut Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.data = Some(serialize_csv_str(&data, false)?);
        Ok(self)
    }

    pub fn csv_str(&mut self, csv: &str) -> &mut Self {
        self.data = Some(csv.to_string());
        self
    }

    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let token = self.api.auth.get_token().await?;
        let body = self.data.as_ref().ok_or(DomoErr("No Data was set to upload".to_string()))?;
        let uri = format!("https://api.domo.com/v1/datasets/{}/data", self.dataset_id);
        let req = self.api.client.put(&uri)
            .bearer_auth(token)
            .header("Content-Type", "text/csv")
            .body(body.clone())
            .send().await?;
        if req.status().is_client_error() {
            let api_err: DomoApiError = req.json().await?;
            error!("{}", api_err);
            Err(Box::new(api_err))
        } else {
            Ok(())
        }
    }
}
pub struct DatasetApiQueryDataBuilder {
    api: Arc<DomoApi>,
    dataset_id: String,
    sql_query: String,
}
impl DatasetApiQueryDataBuilder {
    pub fn new(client: Arc<DomoApi>, dataset_id: &str, sql_query: &str) -> Self {
        Self {
            api: client,
            dataset_id: dataset_id.to_string(),
            sql_query: sql_query.to_string(),
        }
    }
    pub async fn execute(&self) -> Result<DatasetQueryData, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let token = self.api.auth.get_token().await?;
        let body = json!({ "sql": self.sql_query });
        let uri = format!("https://api.domo.com/v1/datasets/query/execute/{}", self.dataset_id);
        let req = self.api.client
            .post(&uri)
            .bearer_auth(token)
            .json(&body)
            .send().await?;
        if req.status().is_client_error() {
            let api_err: DomoApiError = req.json().await?;
            error!("{}", api_err);
            Err(Box::new(api_err))
        } else {
            let data = req.error_for_status()?.json().await?;
            Ok(data)
        }
    }
}
#[derive(Serialize)]
pub struct DatasetApiGetDataBuilder {
    #[serde(skip_serializing)]
    api: Arc<DomoApi>,
    #[serde(skip_serializing)]
    dataset_id: String,
    #[serde(rename(serialize = "includeHeader"))]
    include_headers: bool
}
impl DatasetApiGetDataBuilder {

    pub fn new(client: Arc<DomoApi>, dataset_id: &str) -> Self {
        Self {
            api: client,
            dataset_id: dataset_id.to_string(),
            include_headers: false
        }
    }
    pub fn with_csv_headers(&mut self) -> &mut Self {
        self.include_headers = true;
        self
    }
    pub async fn execute(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let token = self.api.auth.get_token().await?;
        let uri = format!("https://api.domo.com/v1/datasets/{}/data", self.dataset_id);
        let req = self.api.client.get(&uri).bearer_auth(token).send().await?.error_for_status()?;
        let s = req.bytes().await?.to_vec();
        Ok(s)
    }
}

#[derive(Serialize)]
pub struct DatasetApiListBuilder {
    #[serde(skip_serializing)]
    api: Arc<DomoApi>,
    limit: Option<usize>,
    offset: Option<usize>,
    sort: Option<String>,
}

impl DatasetApiListBuilder {
    pub fn new(client: Arc<DomoApi>) -> Self {
        Self {
            api: client,
            limit: Some(50),
            offset: None,
            sort: Some("name".to_string()),
        }
    }
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }
    pub fn sort<S: Into<String>>(&mut self, sort: S) -> &mut Self {
        self.sort = Some(sort.into());
        self
    }
    pub async fn execute(&self) -> Result<Vec<Dataset>,Box<dyn std::error::Error + Send + Sync + 'static>> {
        let token = self.api.auth.get_token().await?;
        let mut query = vec![];
        if let Some(lim) = self.limit {
            query.push(("limit", lim.to_string()));
        }
        if let Some(off) = self.offset {
            query.push(("offset", off.to_string()));
        }
        if let Some(sort) = self.sort.as_ref() {
            query.push(("sort", sort.to_string()));
        }
        let req = self.api.client
            .get("https://api.domo.com/v1/datasets")
            .query(&query)
            .bearer_auth(token)
            .send().await?;
        if req.status().is_client_error() {
            let api_err: DomoApiError = req.json().await?;
            error!("{}", api_err);
            Err(Box::new(api_err))
        } else {
            let s = req.error_for_status()?.json().await?;
            Ok(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DomoClient;

    use super::*;

    #[tokio::test]
    async fn test_dataset_list_builder() {
        let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
        let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");
        
        let domo = DomoClient::new(c, s);
        let datasets = domo.datasets().list().execute().await.unwrap();
        // dbg!(&streams);
        assert_eq!(datasets.len(), 50);
        let five_datasets = domo.datasets().list().limit(5).execute().await.unwrap();
        dbg!(&five_datasets);
        assert_eq!(five_datasets.len(), 5);
    }

    // #[tokio::test]
    // async fn test_dataset_list_builder_threaded() {
    //     let start = std::time::Instant::now();
    //     let c = std::env::var("DOMO_CLIENT_ID").expect("Expected to have Domo client id var set");
    //     let s = std::env::var("DOMO_SECRET").expect("Expected to have Domo secret var set");
    //     let mut ds = vec![];
    //     let mut handles = vec![];
        
    //     let domo = DomoClient::new(c, s);
    //     for thread_num in 0..41 {
    //         let d = domo.clone();
    //         let h = std::thread::spawn(move || smol::block_on(async {
    //             d.datasets().list().limit(5).offset(thread_num * 5).execute().await
    //         }));
    //         handles.push(h);
    //     }
    //     for h in handles {
    //         let mut res = h.join().unwrap().unwrap();
    //         ds.append(&mut res);
    //     }
    //     dbg!(&ds);
    //     println!("Elapsed Time: {:?}", std::time::Instant::now().duration_since(start));
    //     assert_eq!(ds.len(), 205);
    // }
}