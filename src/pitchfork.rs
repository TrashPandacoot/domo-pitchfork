// use crate::domo::activity_log::ActivityLogEntry;
// use crate::domo::dataset::Dataset;
// use crate::domo::group::GroupInfo;
// use crate::domo::page::PageInfo;
// use crate::domo::stream::StreamDataset;
// use crate::domo::user::User;
// use crate::error::{PitchforkError, PitchforkErrorKind};
// use lazy_static::lazy_static;
// use reqwest::Client;
// use reqwest::Method;
// use serde::de::DeserializeOwned;
// use std::marker::PhantomData;

// macro_rules! impl_domo_requests {
//     ($i: ident) => {
//         impl<'t, T> BaseRequest for $i<'t, T>
//         where
//             T: DeserializeOwned,
//         {
//             fn auth(&self) -> &str {
//                 self.auth
//             }
//             fn url(&self) -> &str {
//                 &self.url[..]
//             }
//             fn method(&self) -> Method {
//                 self.method.clone()
//             }
//             fn body(&self) -> Option<String> {
//                 self.body.clone()
//             }
//         }
//         impl<'t, T> From<DomoRequestBuilder<'t, T>> for $i<'t, T>
//         where
//             for<'de> T: DeserializeOwned,
//         {
//             fn from(drb: DomoRequestBuilder<'t, T>) -> Self {
//                 Self {
//                     auth: drb.auth,
//                     method: drb.method,
//                     url: drb.url,
//                     resp_t: PhantomData,
//                     body: drb.body,
//                 }
//             }
//         }
//         impl<'t, T> DomoRequest<T> for $i<'t, T> where for<'de> T: DeserializeOwned {}
//     };
// }
// lazy_static! {
//     /// Static HTTP Client for Domo API
//     #[doc(hidden)]
//     pub static ref CLIENT: Client = Client::new();
// }

// /// `DomoPitchfork` is the top-level object to use to interact with the various Domo APIs
// #[derive(Clone)]
// pub struct DomoPitchfork<'t> {
//     /// Domo Auth Token
//     auth: &'t str,
// }

// impl<'t> DomoPitchfork<'t> {
//     /// Create a new DomoPitchfork with a Domo Auth token
//     pub fn with_token(token: &'t str) -> Self {
//         Self { auth: token }
//     }
//     /// Interact with Domo Datasets API
//     pub fn datasets(&self) -> DatasetsRequestBuilder<'t, Dataset> {
//         DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/datasets/").into()
//     }
//     /// Interact with Domo Streams API
//     pub fn streams(&self) -> StreamsRequestBuilder<'t, StreamDataset> {
//         DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/streams/").into()
//     }
//     /// Interact with Domo Users API
//     pub fn users(&self) -> UsersRequestBuilder<'t, User> {
//         DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/users/").into()
//     }
//     /// Interact with Domo Groups API
//     pub fn groups(&self) -> GroupsRequestBuilder<'t, GroupInfo> {
//         DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/groups/").into()
//     }
//     /// Interact with Domo Pages API
//     pub fn pages(&self) -> PagesRequestBuilder<'t, PageInfo> {
//         DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/pages/").into()
//     }
//     /// Interact with Domo Activity Log API.
//     pub fn audit(&self) -> ActivitiesRequestBuilder<'t, ActivityLogEntry> {
//         DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/audit/").into()
//     }
//     /// Interact with Domo Projects API
//     pub fn projects(&self) -> ProjectsRequestBuilder<'t, ()> {
//         DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/projects/").into()
//     }
//     /// Interact with Domo Accounts API
//     pub fn accounts(&self) -> AccountsRequestBuilder<'t, ()> {
//         DomoRequestBuilder::new(self.auth, "https://api.domo.com/v1/accounts/").into()
//     }
// }

// /// Request Builder for all Dataset API interactions
// pub struct DatasetsRequestBuilder<'t, T: 't>
// where
//     for<'de> T: DeserializeOwned,
// {
//     pub auth: &'t str,
//     pub method: Method,
//     pub url: String,
//     pub resp_t: PhantomData<*const T>,
//     pub body: Option<String>,
// }

// /// Request Builder for all Stream API interactions
// pub struct StreamsRequestBuilder<'t, T: 't>
// where
//     for<'de> T: DeserializeOwned,
// {
//     pub auth: &'t str,
//     pub method: Method,
//     pub url: String,
//     pub resp_t: PhantomData<*const T>,
//     pub body: Option<String>,
// }
// /// Request Builder for all User API interactions
// pub struct UsersRequestBuilder<'t, T: 't>
// where
//     for<'de> T: DeserializeOwned,
// {
//     pub auth: &'t str,
//     pub method: Method,
//     pub url: String,
//     pub resp_t: PhantomData<*const T>,
//     pub body: Option<String>,
// }
// /// Request Builder for all Group API interactions
// pub struct GroupsRequestBuilder<'t, T: 't>
// where
//     for<'de> T: DeserializeOwned,
// {
//     pub auth: &'t str,
//     pub method: Method,
//     pub url: String,
//     pub resp_t: PhantomData<*const T>,
//     pub body: Option<String>,
// }
// /// Request Builder for all Page API interactions
// pub struct PagesRequestBuilder<'t, T: 't>
// where
//     for<'de> T: DeserializeOwned,
// {
//     pub auth: &'t str,
//     pub method: Method,
//     pub url: String,
//     pub resp_t: PhantomData<*const T>,
//     pub body: Option<String>,
// }
// /// Request Builder for all Activity Log API interactions
// pub struct ActivitiesRequestBuilder<'t, T: 't>
// where
//     for<'de> T: DeserializeOwned,
// {
//     pub auth: &'t str,
//     pub method: Method,
//     pub url: String,
//     pub resp_t: PhantomData<*const T>,
//     pub body: Option<String>,
// }
// /// Request Builder for all Account API interactions
// pub struct AccountsRequestBuilder<'t, T: 't>
// where
//     for<'de> T: DeserializeOwned,
// {
//     pub auth: &'t str,
//     pub method: Method,
//     pub url: String,
//     pub resp_t: PhantomData<*const T>,
//     pub body: Option<String>,
// }
// /// Request Builder for all Project and Task API interactions
// pub struct ProjectsRequestBuilder<'t, T: 't>
// where
//     for<'de> T: DeserializeOwned,
// {
//     pub auth: &'t str,
//     pub method: Method,
//     pub url: String,
//     pub resp_t: PhantomData<*const T>,
//     pub body: Option<String>,
// }
// impl_domo_requests!(StreamsRequestBuilder);
// impl_domo_requests!(DatasetsRequestBuilder);
// impl_domo_requests!(UsersRequestBuilder);
// impl_domo_requests!(GroupsRequestBuilder);
// impl_domo_requests!(PagesRequestBuilder);
// impl_domo_requests!(ActivitiesRequestBuilder);
// impl_domo_requests!(AccountsRequestBuilder);
// impl_domo_requests!(ProjectsRequestBuilder);
// pub struct DomoRequestBuilder<'t, T: 't>
// where
//     for<'de> T: DeserializeOwned,
// {
//     pub auth: &'t str,
//     pub method: Method,
//     pub url: String,
//     pub resp_t: PhantomData<*const T>,
//     pub body: Option<String>,
// }

// impl<'t, T> DomoRequestBuilder<'t, T>
// where
//     T: DeserializeOwned,
// {
//     pub fn new<S>(auth: &'t str, url: S) -> DomoRequestBuilder<'t, T>
//     where
//         for<'de> S: Into<String>,
//         T: DeserializeOwned,
//     {
//         DomoRequestBuilder {
//             auth,
//             method: Method::GET,
//             url: url.into(),
//             resp_t: PhantomData,
//             body: None,
//         }
//     }
// }

// impl<'t, T> BaseRequest for DomoRequestBuilder<'t, T>
// where
//     T: DeserializeOwned,
// {
//     fn url(&self) -> &str {
//         &self.url[..]
//     }
//     fn auth(&self) -> &str {
//         self.auth
//     }
//     fn method(&self) -> Method {
//         self.method.clone()
//     }
//     fn body(&self) -> Option<String> {
//         self.body.clone()
//     }
// }

// impl<'t, T> DomoRequest<T> for DomoRequestBuilder<'t, T> where for<'de> T: DeserializeOwned {}

// /// Base level request info.
// pub trait BaseRequest {
//     fn url(&self) -> &str;
//     fn auth(&self) -> &str;
//     fn method(&self) -> Method;
//     fn body(&self) -> Option<String>;
// }

// /// Defines Domo Requests
// pub trait DomoRequest<T>: BaseRequest {
//     fn run(&self) -> Result<T, PitchforkError>
//     where
//         for<'de> T: DeserializeOwned,
//     {
//         let mut response = CLIENT
//             .request(self.method(), self.url())
//             .bearer_auth(self.auth())
//             .header("Content-Type", "application/json")
//             .body(self.body().take().unwrap_or_default())
//             .send()
//             .expect("ಠ_ಠ you just got Domo'd");

//         if response.status().is_success() {
//             let res: T = response.json()?;
//             Ok(res)
//         } else {
//             eprintln!("response: {:?}", &response);
//             let code = response.status().as_u16();
//             Err(PitchforkErrorKind::DomoBadRequest(code, response.text()?).into())
//         }
//     }
//     fn retrieve_and_deserialize_json(&self) -> Result<T, PitchforkError>
//     where
//         for<'de> T: DeserializeOwned,
//     {
//         let mut response = CLIENT
//             .request(self.method(), self.url())
//             .bearer_auth(self.auth())
//             .header("Content-Type", "application/json")
//             .body(self.body().take().unwrap_or_default())
//             .send()
//             .expect("ಠ_ಠ you just got Domo'd");

//         if response.status().is_success() {
//             //            let mut body = vec![];
//             //            std::io::copy(&mut response, &mut body);
//             //            println!("{}", String::from_utf8(body).unwrap());
//             let res: T = response.json()?;
//             Ok(res)
//         } else {
//             let code = response.status().as_u16();
//             Err(PitchforkErrorKind::DomoBadRequest(code, response.text()?).into())
//         }
//     }

//     fn send_csv(&self) -> Result<reqwest::Response, PitchforkError> {
//         let mut response = CLIENT
//             .request(self.method(), self.url())
//             .bearer_auth(self.auth())
//             .header("Content-Type", "text/csv")
//             .body(self.body().take().unwrap_or_default())
//             .send()?;
//         if response.status().is_success() {
//             Ok(response)
//         } else {
//             let code = response.status().as_u16();
//             Err(PitchforkErrorKind::DomoBadRequest(code, response.text()?).into())
//         }
//     }
//     fn send_json(&self) -> Result<reqwest::Response, PitchforkError> {
//         let mut response = CLIENT
//             .request(self.method(), self.url())
//             .bearer_auth(self.auth())
//             .header("Content-Type", "application/json")
//             .body(self.body().take().unwrap_or_default())
//             .send()?;
//         if response.status().is_success() {
//             Ok(response)
//         } else {
//             let code = response.status().as_u16();
//             Err(PitchforkErrorKind::DomoBadRequest(code, response.text()?).into())
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::auth::DomoClientAppCredentials;
//     use std::env;
//     #[test]
//     fn test_dataset_list() {
//         let domo_client_id = env::var("DOMO_CLIENT_ID").expect("No DOMO_CLIENT_ID env var found");
//         let domo_secret = env::var("DOMO_SECRET").expect("No DOMO_SECRET env var found");
//         let client_creds = DomoClientAppCredentials::default()
//             .client_id(&domo_client_id)
//             .client_secret(&domo_secret)
//             .build();
//         let token = client_creds.get_access_token();
//         let domo = DomoPitchfork::with_token(&token);
//         let ds_list = domo.datasets().list(5, 0);
//         match ds_list {
//             Ok(ds) => {
//                 println!("{:?}", ds);
//                 assert_eq!(ds.len(), 5);
//             }
//             Err(e) => println!("{}", e),
//         };
//     }

//     #[test]
//     fn test_dataset_query() {
//         let domo_client_id = env::var("DOMO_CLIENT_ID").expect("No DOMO_CLIENT_ID env var found");
//         let domo_secret = env::var("DOMO_SECRET").expect("No DOMO_SECRET env var found");
//         let client_creds = DomoClientAppCredentials::default()
//             .client_id(&domo_client_id)
//             .client_secret(&domo_secret)
//             .build();
//         let token = client_creds.get_access_token();
//         let domo = DomoPitchfork::with_token(&token);
//         let dq = domo.datasets().query_data(
//             "9e325a09-e7da-42b3-a34f-f96a25928d81",
//             "SELECT * FROM table WHERE `Order Priority` = 'High'",
//         );
//         match dq {
//             Ok(ds) => {
//                 println!("{:?}", ds);
//                 assert_eq!(ds.columns.len(), 3);
//                 assert_eq!(ds.num_rows, 4);
//             }
//             Err(e) => println!("{}", e),
//         };
//     }
// }
