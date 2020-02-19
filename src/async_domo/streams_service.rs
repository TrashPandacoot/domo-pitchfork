use crate::auth::DomoClientAuth;
use crate::domo::dataset::Dataset;
use crate::domo::stream::StreamDataset;
use crate::domo::stream::StreamDatasetSchema;
use crate::domo::stream::StreamExecution;
use crate::domo::stream::StreamSearchQuery;
use crate::domo::stream::UpdateMethod;
use crate::{PitchforkError, PitchforkErrorKind};
use serde::Serialize;

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

    pub async fn delete(&self, stream_id: usize) -> Result<(), PitchforkError> {
        unimplemented!()
    }
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
    pub async fn execution(
        &self,
        stream_id: usize,
        execution_id: usize,
    ) -> Result<StreamExecution, PitchforkError> {
        unimplemented!()
    }
    pub async fn list_executions(
        &self,
        stream_id: usize,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<StreamExecution>, PitchforkError> {
        unimplemented!()
    }
    pub async fn commit_execution(
        &self,
        stream_id: usize,
        execution_id: usize,
    ) -> Result<StreamExecution, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/executions/{exid}",
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
    pub async fn abort_execution(
        &self,
        stream_id: usize,
        execution_id: usize,
    ) -> Result<(), PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}/{id}/executions/{exid}",
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
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
// }
