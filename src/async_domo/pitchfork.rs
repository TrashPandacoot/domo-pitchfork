use crate::auth::DomoClientAuth;
use crate::auth::DomoToken;
use crate::domo::dataset::Dataset;
use crate::domo::dataset::DatasetQueryData;
use crate::domo::dataset::DatasetSchema;
use crate::PitchforkError;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
// pub struct DomoPitchfork {
//     pub base_url: String,
//     pub user_agent: String,
// }

pub struct DomoDatasetPitchfork {
    pub base_url: String,
    pub user_agent: String,
    api_version: String,
    auth: DomoClientAuth,
    client: reqwest::Client,
}

impl Default for DomoDatasetPitchfork {
    fn default() -> Self {
        Self::new()
    }
}
impl DomoDatasetPitchfork {
    #[must_use]
    pub fn new() -> Self {
        Self {
            base_url: "https://api.domo.com".to_string(),
            user_agent: "Domo Pitchfork".to_string(),
            api_version: "v1/datasets".to_string(),
            auth: DomoClientAuth::default().with_data_scope(),
            client: reqwest::Client::new(),
        }
    }
    pub async fn list(&self) -> Result<Vec<Dataset>, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}?limit={limit}&offset={offset}",
            base_url = self.base_url,
            api_ver = self.api_version,
            limit = 50,
            offset = 0
        );
        self.auth.auth().await?;
        if let Some(token) = self.auth.auth.borrow().as_ref() {
            let resp = self
                .client
                .get(&url)
                .bearer_auth(&token.domo_token.access_token)
                .send()
                .await?
                .json::<Vec<Dataset>>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API List",
            ))
        }
    }

    pub async fn info(&mut self, dataset_id: &str) -> Result<Dataset, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = dataset_id
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .get(&url)
                .bearer_auth(token)
                .send()
                .await?
                .json::<Dataset>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API List",
            ))
        }
    }

    pub async fn create(&mut self, ds_meta: &DatasetSchema) -> Result<Dataset, PitchforkError> {
        unimplemented!()
    }
    pub async fn query_data(
        &mut self,
        dataset_id: &str,
        sql_query: &str,
    ) -> Result<DatasetQueryData, PitchforkError> {
        unimplemented!()
    }
    pub async fn download_data(
        &mut self,
        dataset_id: &str,
        include_csv_headers: bool,
    ) -> Result<String, PitchforkError> {
        unimplemented!()
    }
    pub async fn get_data<T: DeserializeOwned>(
        &mut self,
        dataset_id: &str,
    ) -> Result<Vec<T>, PitchforkError> {
        unimplemented!()
    }
    pub async fn upload_data_from_str(
        &mut self,
        dataset_id: &str,
        csv: &str,
    ) -> Result<(), PitchforkError> {
        unimplemented!()
    }
    pub async fn upload_data<T: Serialize>(
        &mut self,
        dataset_id: &str,
        data: &[T],
    ) -> Result<(), PitchforkError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_ds() -> Result<(), Box<dyn std::error::Error>> {
        // let cid = std::env::var("DOMO_CLIENT_ID").expect("expected domo client id var");
        // let cs = std::env::var("DOMO_SECRET").expect("expected domo secret var");
        let d = DomoDatasetPitchfork::new();
        let list: Vec<Dataset> = d.list().await?;
        let list2: Vec<Dataset> = d.list().await?;
        assert_eq!(list.len(), 50);
        assert_eq!(list.len(), list2.len());
        Ok(())
    }
    #[tokio::test]
    async fn test_info_ds() -> Result<(), Box<dyn std::error::Error>> {
        // let cid = std::env::var("DOMO_CLIENT_ID").expect("expected domo client id var");
        // let cs = std::env::var("DOMO_SECRET").expect("expected domo secret var");
        let mut d = DomoDatasetPitchfork::new();
        let ds = d.info("385c64fb-0c36-492d-8c0e-dc409e90895d").await?;
        assert_eq!(ds.id, "385c64fb-0c36-492d-8c0e-dc409e90895d");
        Ok(())
    }
}
