pub struct DatasetService {}

impl DatasetService {
    pub async fn list() -> Result<Vec<Dataset>, PitchforkError> {
        let url = format!(
            "{base_url}/{api_ver}?limit={limit}&offset={offset}",
            base_url = "https://api.domo.com",
            api_ver = "v1/datasets",
            limit = 50,
            offset = 0
        );
        let resp = reqwest::get(url).await?.json::<Dataset>().await?;
        Ok(resp)
    }
}
