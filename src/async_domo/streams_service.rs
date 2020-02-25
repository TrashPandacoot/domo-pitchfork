use crate::auth::DomoClientAuth;
use crate::domo::dataset::Dataset;
use crate::domo::stream::StreamDataset;
use crate::domo::stream::StreamDatasetSchema;
use crate::domo::stream::StreamExecution;
use crate::domo::stream::StreamSearchQuery;
use crate::domo::stream::UpdateMethod;
use crate::{PitchforkError, PitchforkErrorKind};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct DomoStreamPitchfork {
    pub base_url: String,
    pub user_agent: String,
    api_version: String,
    auth: DomoClientAuth,
    client: reqwest::Client,
    execution_id: usize,
    part_id: usize,
}

impl Default for DomoStreamPitchfork {
    fn default() -> Self {
        Self::new()
    }
}
impl DomoStreamPitchfork {
    #[must_use]
    pub fn new() -> Self {
        Self {
            base_url: "https://api.domo.com".to_string(),
            user_agent: "Domo Pitchfork".to_string(),
            api_version: "v1/streams".to_string(),
            auth: DomoClientAuth::default().with_data_scope(),
            client: reqwest::Client::new(),
            execution_id: 0,
            part_id: 0,
        }
    }
    #[must_use]
    pub fn with_credentials(client_id: &str, client_secret: &str) -> Self {
        Self {
            base_url: "https://api.domo.com".to_string(),
            user_agent: "Domo Pitchfork".to_string(),
            api_version: "v1/streams".to_string(),
            auth: DomoClientAuth::default()
                .with_data_scope()
                .client_id(client_id)
                .client_secret(client_secret),
            client: reqwest::Client::new(),
            execution_id: 0,
            part_id: 0,
        }
    }

    /// List Domo Streams.
    /// Max limit is 500.
    /// Offset is the offset of the Stream ID to begin list of streams within the response
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn list(
        &self,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<StreamDataset>, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}?limit={limit}&offset={offset}",
            base_url = self.base_url,
            api_ver = self.api_version,
            limit = limit,
            offset = offset
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .get(&url)
                .bearer_auth(token)
                .send()
                .await?
                .error_for_status()?
                .json::<Vec<StreamDataset>>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Stream API List",
            ))
        }
    }

    /// Retrieve details for a given Domo Stream.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn info(&self, stream_id: usize) -> Result<StreamDataset, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = stream_id
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .get(&url)
                .bearer_auth(token)
                .send()
                .await?
                .error_for_status()?
                .json::<StreamDataset>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Streams API",
            ))
        }
    }

    /// Returns a list of [`StreamDataset`]s that meet the search query criteria.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn search(
        &self,
        query: &StreamSearchQuery,
    ) -> Result<Vec<StreamDataset>, PitchforkError> {
        // TODO: optional fields query param
        let q = match query {
            StreamSearchQuery::DatasetId(s) => format!("dataSource.id:{}", s),
            StreamSearchQuery::DatasetOwnerId(user_id) => {
                format!("dataSource.owner.id:{}", user_id)
            }
        };
        let url = format!(
            "{base_url}/{api_ver}/search?q={q}&fields=all",
            base_url = self.base_url,
            api_ver = self.api_version,
            q = q,
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .post(&url)
                .bearer_auth(token)
                .send()
                .await?
                .error_for_status()?
                .json::<Vec<StreamDataset>>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Streams API",
            ))
        }
    }

    /// Create a new `StreamDataset` to create executions and upload data to.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn create(
        &self,
        ds_meta: &StreamDatasetSchema,
    ) -> Result<StreamDataset, PitchforkError> {
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
                .error_for_status()?
                .json::<StreamDataset>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Streams API",
            ))
        }
    }

    /// Delete a given Domo Stream.
    /// Warning: this action is destructive and cannot be reversed.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn delete(&self, stream_id: usize) -> Result<(), PitchforkError> {
        unimplemented!()
    }

    /// Updates Stream Update Method settings.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn change_stream_update_method(
        &self,
        stream_id: usize,
        update_method: &UpdateMethod,
    ) -> Result<Dataset, PitchforkError> {
        unimplemented!()
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
    }

    /// Create a `StreamExecution` to upload data parts to and update the data in Domo.
    /// Warning: Creating an Execution on a Stream will abort all other Executions on that Stream.
    /// Each Stream can only have one active Execution at a time.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn create_execution(
        &self,
        stream_id: usize,
    ) -> Result<StreamExecution, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/executions",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = stream_id,
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .post(&url)
                .bearer_auth(token)
                .send()
                .await?
                .error_for_status()?
                .json::<StreamExecution>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Streams API",
            ))
        }
    }

    /// Details for a `StreamExecution` for a given `StreamDataset`
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn execution(
        &self,
        stream_id: usize,
        execution_id: usize,
    ) -> Result<StreamExecution, PitchforkError> {
        unimplemented!()
    }

    /// List Domo Executions for a given Domo Stream.
    /// Max limit is 500.
    /// Offset is the offset of the Stream ID to begin list of streams within the response
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn list_executions(
        &self,
        stream_id: usize,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<StreamExecution>, PitchforkError> {
        unimplemented!()
    }

    /// Commit a stream execution and finalize insertion of dataparts into Domo Stream Dataset.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn commit_execution(
        &self,
        stream_id: usize,
        execution_id: usize,
    ) -> Result<StreamExecution, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/executions/{exid}/commit",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = stream_id,
            exid = execution_id,
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let resp = self
                .client
                .put(&url)
                .bearer_auth(token)
                .send()
                .await?
                .error_for_status()?
                .json::<StreamExecution>()
                .await?;
            Ok(resp)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Streams API",
            ))
        }
    }

    /// Abort a stream execution in progress and discard all data parts uploaded to the execution.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn abort_execution(
        &self,
        stream_id: usize,
        execution_id: usize,
    ) -> Result<(), PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/executions/{exid}/abort",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = stream_id,
            exid = execution_id,
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let _ = self
                .client
                .put(&url)
                .bearer_auth(token)
                .send()
                .await?
                .error_for_status()?;
            Ok(())
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Streams API",
            ))
        }
    }

    /// Upload a data part to a stream execution in progress where the data part
    /// is a `Serializable` vec of T.
    /// Parts can be uploaded simultaneously and in any order.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn upload<T: Serialize>(
        &self,
        stream_id: usize,
        execution_id: usize,
        part: usize,
        data: &[T],
    ) -> Result<StreamExecution, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/executions/{exid}/part/{partNum}",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = stream_id,
            exid = execution_id,
            partNum = part,
        );
        let body = crate::util::csv::serialize_to_csv_str(data, false)
            .map_err(|e| PitchforkError::from(e).with_kind(PitchforkErrorKind::Csv))?;
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let se = self
                .client
                .put(&url)
                .bearer_auth(token)
                .body(body)
                .header("Content-Type", "text/csv")
                .send()
                .await?
                .error_for_status()?
                .json::<StreamExecution>()
                .await?;
            Ok(se)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }

    /// Upload a data part to a stream execution in progress.
    /// Parts can be uploaded simultaneously and in any order.
    ///
    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn upload_from_str(
        &self,
        stream_id: usize,
        execution_id: usize,
        part: usize,
        csv_part: &str,
    ) -> Result<StreamExecution, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/executions/{exid}/part/{partNum}",
            base_url = self.base_url,
            api_ver = self.api_version,
            id = stream_id,
            exid = execution_id,
            partNum = part,
        );
        self.auth.authenticate().await?;
        if let Some(token) = self.auth.bearer_token() {
            let se = self
                .client
                .put(&url)
                .bearer_auth(token)
                .body(csv_part.to_string())
                .header("Content-Type", "text/csv")
                .send()
                .await?
                .error_for_status()?
                .json::<StreamExecution>()
                .await?;
            Ok(se)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }

    // Dataset API Methods

    /// # Errors
    ///
    /// Returns `PitchforkError` if HTTP request to Domo API fails.
    pub async fn get_data<T: DeserializeOwned>(
        &self,
        dataset_id: &str,
    ) -> Result<Vec<T>, PitchforkError> {
        let url = format!(
            "{base_url}/v1/datasets/{id}/data?includeHeader=true",
            base_url = self.base_url,
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
                .text()
                .await?;
            let data = crate::util::csv::deserialize_csv_str(&resp)?;
            Ok(data)
        } else {
            Err(PitchforkError::from(
                "Failed to Authenticate with Domo Datasets API",
            ))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
// }
