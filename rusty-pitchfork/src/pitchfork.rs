use crate::domo::page::Page;
use crate::domo::group::Group;
use crate::domo::user::User;
use crate::domo::dataset::Dataset;
use crate::domo::stream::StreamDataset;
use crate::error::DomoError;
use lazy_static::lazy_static;
use reqwest::Client;
use reqwest::Method;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

macro_rules! ImplDomoRequests {
    ($i: ident) => {
        // pub struct $i<'t, T: 't>
        //     where for<'de> T: DeserializeOwned
        // {
        //     pub auth: &'t str,
        //     pub method: Method,
        //     pub url: String,
        //     pub resp_t: PhantomData<*const T>,
        //     pub body: Option<String>,
        // }
        impl<'t, T> BaseRequest for $i<'t, T>
        where
            T: DeserializeOwned,
        {
            fn auth(&self) -> &str {
                self.auth
            }
            fn url(&self) -> &str {
                &self.url[..]
            }
            fn method(&self) -> Method {
                self.method.clone()
            }
            fn body(&self) -> Option<String> {
                self.body.clone()
            }
        }
        impl<'t, T> From<DomoRequestBuilder<'t, T>> for $i<'t, T>
        where
            for<'de> T: DeserializeOwned,
        {
            fn from(drb: DomoRequestBuilder<'t, T>) -> Self {
                Self {
                    auth: drb.auth,
                    method: drb.method,
                    url: drb.url,
                    resp_t: PhantomData,
                    body: drb.body,
                }
            }
        }
        impl<'t, T> DomoRequest<T> for $i<'t, T> where for<'de> T: DeserializeOwned {}
    };
}
lazy_static! {
    /// Static HTTP Client for Domo API
    pub static ref CLIENT: Client = Client::new();
}
#[derive(Clone)]
pub struct DomoPitchfork<'t> {
    auth: &'t str,
}

impl<'t> DomoPitchfork<'t> {
    pub fn with_token(token: &'t str) -> Self {
        Self { auth: token }
    }
    /// Interact with Domo Datasets API
    pub fn datasets(&self) -> DatasetsRequestBuilder<'t, Dataset> {
        DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/datasets/").into()
    }
    /// Interact with Domo Streams API
    pub fn streams(&self) -> StreamsRequestBuilder<'t, StreamDataset> {
        DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/streams/").into()
    }
    /// Interact with Domo Users API
    pub fn users(&self) -> UsersRequestBuilder<'t, User> {
        DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/users/").into()
    }
    /// Interact with Domo Groups API
    pub fn groups(&self) -> GroupsRequestBuilder<'t, Group> {
        DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/groups/").into()
    }
    /// Interact with Domo Pages API
    pub fn pages(&self) -> PagesRequestBuilder<'t, Page> {
        DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/pages/").into()
    }
    /// Interact with Domo Projects API
    pub fn projects(&self) -> ProjectsRequestBuilder<'t, ()> {
        DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/projects/").into()
    }
    /// Interact with Domo Accounts API
    pub fn accounts(&self) -> AccountsRequestBuilder<'t, ()> {
        DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/accounts/").into()
    }
}

pub struct DatasetsRequestBuilder<'t, T: 't>
where
    for<'de> T: DeserializeOwned,
{
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}
pub struct StreamsRequestBuilder<'t, T: 't>
where
    for<'de> T: DeserializeOwned,
{
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}
pub struct UsersRequestBuilder<'t, T: 't>
where
    for<'de> T: DeserializeOwned,
{
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}
pub struct GroupsRequestBuilder<'t, T: 't>
where
    for<'de> T: DeserializeOwned,
{
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}
pub struct PagesRequestBuilder<'t, T: 't>
where
    for<'de> T: DeserializeOwned,
{
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}
pub struct ActivitiesRequestBuilder<'t, T: 't>
where
    for<'de> T: DeserializeOwned,
{
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}
pub struct AccountsRequestBuilder<'t, T: 't>
where
    for<'de> T: DeserializeOwned,
{
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}
pub struct ProjectsRequestBuilder<'t, T: 't>
where
    for<'de> T: DeserializeOwned,
{
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}
ImplDomoRequests!(StreamsRequestBuilder);
ImplDomoRequests!(DatasetsRequestBuilder);
ImplDomoRequests!(UsersRequestBuilder);
ImplDomoRequests!(GroupsRequestBuilder);
ImplDomoRequests!(PagesRequestBuilder);
ImplDomoRequests!(ActivitiesRequestBuilder);
ImplDomoRequests!(AccountsRequestBuilder);
ImplDomoRequests!(ProjectsRequestBuilder);
pub struct DomoRequestBuilder<'t, T: 't>
where
    for<'de> T: DeserializeOwned,
{
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}

impl<'t, T> DomoRequestBuilder<'t, T>
where
    T: DeserializeOwned,
{
    pub fn new<S>(auth: &'t str, url: S) -> DomoRequestBuilder<'t, T>
    where
        for<'de> S: Into<String>,
        T: DeserializeOwned,
    {
        DomoRequestBuilder {
            auth,
            method: Method::GET,
            url: url.into(),
            resp_t: PhantomData,
            body: None,
        }
    }
}

impl<'t, T> BaseRequest for DomoRequestBuilder<'t, T>
where
    T: DeserializeOwned,
{
    fn auth(&self) -> &str {
        self.auth
    }
    fn url(&self) -> &str {
        &self.url[..]
    }
    fn method(&self) -> Method {
        self.method.clone()
    }
    fn body(&self) -> Option<String> {
        self.body.clone()
    }
}

impl<'t, T> DomoRequest<T> for DomoRequestBuilder<'t, T> where for<'de> T: DeserializeOwned {}

pub trait BaseRequest {
    fn url(&self) -> &str;
    fn auth(&self) -> &str;
    fn method(&self) -> Method;
    fn body(&self) -> Option<String>;
}

pub trait DomoRequest<T>: BaseRequest {
    fn run(&self) -> Result<T, DomoError>
    where
        for<'de> T: DeserializeOwned,
    {
        let mut response = CLIENT
            .request(self.method(), self.url())
            .bearer_auth(self.auth())
            .header("Content-Type", "application/json")
            .body(self.body().take().unwrap_or_default())
            .send()
            .expect("ಠ_ಠ you just got Domo'd");

        // let mut buf = String::new();
        // response
        //     .read_to_string(&mut buf)
        //     .expect("ಠ_ಠ failed to read response");
        if response.status().is_success() {
            let res: T = response.json()?;
            Ok(res)
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
    fn retrieve_and_deserialize_json(&self) -> Result<T, DomoError>
    where
        for<'de> T: DeserializeOwned,
    {
        let mut response = CLIENT
            .request(self.method(), self.url())
            .bearer_auth(self.auth())
            .header("Content-Type", "application/json")
            .body(self.body().take().unwrap_or_default())
            .send()
            .expect("ಠ_ಠ you just got Domo'd");

        // let mut buf = String::new();
        // response
        //     .read_to_string(&mut buf)
        //     .expect("ಠ_ಠ failed to read response");
        if response.status().is_success() {
            let res: T = response.json()?;
            Ok(res)
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

    fn send_csv(&self) -> Result<reqwest::Response, DomoError> {
        let response = CLIENT
            .request(self.method(), self.url())
            .bearer_auth(self.auth())
            .header("Content-Type", "text/csv")
            .body(self.body().take().unwrap_or_default())
            .send()?;
        Ok(response)
    }
    fn send_json(&self) -> Result<reqwest::Response, DomoError> {
        let response = CLIENT
            .request(self.method(), self.url())
            .bearer_auth(self.auth())
            .header("Content-Type", "application/json")
            .body(self.body().take().unwrap_or_default())
            .send()?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::DomoClientAppCredentials;
    use crate::domo::dataset::*;
    use std::env;
    #[test]
    fn test_dataset_list() {
        let domo_client_id = env::var("DOMO_CLIENT_ID").expect("No DOMO_CLIENT_ID env var found");
        let domo_secret = env::var("DOMO_SECRET").expect("No DOMO_SECRET env var found");
        let client_creds = DomoClientAppCredentials::default()
            .client_id(&domo_client_id)
            .client_secret(&domo_secret)
            .build();
        let token = client_creds.get_access_token();
        let domo = DomoManager::with_token(&token);
        let ds_list = domo.datasets().list(5, 0);
        let s_list = domo.streams().list(1, 0);
        match ds_list {
            Ok(ds) => {
                println!("{:?}", ds);
                assert_eq!(ds.len(), 5);
            }
            Err(e) => println!("{}", e),
        };
    }

    #[test]
    fn test_dataset_query() {
        let domo_client_id = env::var("DOMO_CLIENT_ID").expect("No DOMO_CLIENT_ID env var found");
        let domo_secret = env::var("DOMO_SECRET").expect("No DOMO_SECRET env var found");
        let client_creds = DomoClientAppCredentials::default()
            .client_id(&domo_client_id)
            .client_secret(&domo_secret)
            .build();
        let token = client_creds.get_access_token();
        let domo = DomoManager::with_token(&token);
        let dq = domo.datasets().query_data(
            "9e325a09-e7da-42b3-a34f-f96a25928d81",
            "SELECT * FROM table WHERE `Order Priority` = 'High'",
        );
        match dq {
            Ok(ds) => {
                println!("{:?}", ds);
                assert_eq!(ds.columns.len(), 3);
                assert_eq!(ds.num_rows, 4);
            }
            Err(e) => println!("{}", e),
        };
    }
}
