use crate::auth::DomoClientAuth;
use crate::domo::dataset::Dataset;
use crate::domo::dataset::DatasetQueryData;
use crate::domo::dataset::DatasetSchema;
use crate::domo::dataset::Schema;
use crate::{PitchforkError, PitchforkErrorKind};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

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
    #[must_use]
    pub fn with_credentials(client_id: &str, client_secret: &str) -> Self {
        Self {
            base_url: "https://api.domo.com".to_string(),
            user_agent: "Domo Pitchfork".to_string(),
            api_version: "v1/datasets".to_string(),
            auth: DomoClientAuth::default()
                .with_data_scope()
                .client_id(client_id)
                .client_secret(client_secret),
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
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .get(&url)
                .bearer_auth(token)
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

    pub async fn info(&self, dataset_id: &str) -> Result<Dataset, PitchforkError> {
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
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }

    pub async fn create(&self, ds_meta: &DatasetSchema) -> Result<Dataset, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/",
            base_url = self.base_url,
            api_ver = self.api_version,
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .post(&url)
                .bearer_auth(token)
                .json(ds_meta)
                .send()
                .await?
                .json::<Dataset>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }

    pub async fn update_dataset_name(
        &self,
        dataset_id: &str,
        new_name: &str,
    ) -> Result<Dataset, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = dataset_id
        );
        let body = json!({ "name": new_name });
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .post(&url)
                .bearer_auth(token)
                .json(&body)
                .send()
                .await?
                .json::<Dataset>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }
    pub async fn update_dataset_description(
        &self,
        dataset_id: &str,
        new_description: &str,
    ) -> Result<Dataset, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = dataset_id
        );
        let body = json!({ "description": new_description });
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .post(&url)
                .bearer_auth(token)
                .json(&body)
                .send()
                .await?
                .json::<Dataset>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }
    pub async fn update_dataset_schema(
        &self,
        dataset_id: &str,
        schema: Schema,
    ) -> Result<Dataset, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = dataset_id
        );
        let body = json!({ "schema": schema });
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .post(&url)
                .bearer_auth(token)
                .json(&body)
                .send()
                .await?
                .json::<Dataset>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }

    pub async fn query_data(
        &self,
        dataset_id: &str,
        sql_query: &str,
    ) -> Result<DatasetQueryData, PitchforkError> {
        let body = json!({ "sql": sql_query }).to_string();
        let url = format!(
            "{base_url}/{api_ver}/query/execute/{id}",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = dataset_id
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .post(&url)
                .bearer_auth(token)
                .body(body)
                .send()
                .await?
                .json::<DatasetQueryData>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }
    pub async fn download_data(
        &self,
        dataset_id: &str,
        include_csv_headers: bool,
    ) -> Result<String, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/data?includeHeader={headers}",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = dataset_id,
            headers = include_csv_headers
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .get(&url)
                .bearer_auth(token)
                .send()
                .await?
                .text()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }
    pub async fn get_data<T: DeserializeOwned>(
        &self,
        dataset_id: &str,
    ) -> Result<Vec<T>, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/data?includeHeader=true",
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
                .json::<Vec<T>>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }
    pub async fn upload_data_from_str(
        &self,
        dataset_id: &str,
        csv: &str,
    ) -> Result<(), PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/data",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = dataset_id
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let _ = self
                .client
                .post(&url)
                .bearer_auth(token)
                .body(csv.to_string())
                .send()
                .await?
                .error_for_status()?;
            Ok(())
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }
    pub async fn upload_data<T: Serialize>(
        &self,
        dataset_id: &str,
        data: &[T],
    ) -> Result<(), PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/data",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = dataset_id
        );
        let body = crate::util::csv::serialize_to_csv_str(data, false)
            .map_err(|e| PitchforkError::from(e).with_kind(PitchforkErrorKind::Csv))?;
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let _ = self
                .client
                .post(&url)
                .bearer_auth(token)
                .body(body)
                .send()
                .await?
                .error_for_status()?;
            Ok(())
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
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
        let d = DomoDatasetPitchfork::new();
        let ds = d.info("385c64fb-0c36-492d-8c0e-dc409e90895d").await?;
        assert_eq!(ds.id, "385c64fb-0c36-492d-8c0e-dc409e90895d");
        Ok(())
    }

    #[tokio::test]
    async fn test_creating_with_creds() -> Result<(), Box<dyn std::error::Error>> {
        let cid = std::env::var("DOMO_CLIENT_ID").expect("expected domo client id var");
        let cs = std::env::var("DOMO_SECRET").expect("expected domo secret var");
        let d = DomoDatasetPitchfork::with_credentials(&cid, &cs);
        let list = d.list().await?;
        assert_eq!(list.len(), 50);
        Ok(())
    }
}
