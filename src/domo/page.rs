// use crate::error::PitchforkError;
// use crate::pitchfork::{DomoRequest, PagesRequestBuilder};
// use log::debug;
// use reqwest::Method;
// use serde::{Deserialize, Serialize};
// use std::marker::PhantomData;

// // [Page Object](https://developer.domo.com/docs/page-api-reference/page)
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PageInfo {
//     pub name: String,
//     pub id: String,
//     #[serde(rename = "parentId")]
//     pub parent_id: u64,
//     #[serde(rename = "ownerId")]
//     pub owner_id: u64,
//     pub locked: bool,
//     #[serde(rename = "collectionIds")]
//     pub collection_ids: Vec<u64>,
//     #[serde(rename = "cardIds")]
//     pub card_ids: Vec<u64>,
//     pub children: Vec<Page>,
//     pub visibility: PageVisibility,
//     #[serde(rename = "userIds")]
//     pub user_ids: Vec<u64>,
//     #[serde(rename = "pageIds")]
//     pub page_ids: Vec<u64>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Page {
//     pub id: u64,
//     pub name: String,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PageVisibility {
//     #[serde(rename = "userIds")]
//     pub user_ids: Vec<u64>,
//     #[serde(rename = "pageIds")]
//     pub page_ids: Vec<u64>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct PageCollection {
//     pub id: u64,
//     pub title: String,
//     pub description: String,
//     #[serde(rename = "cardIds")]
//     pub card_ids: Vec<u64>,
// }
// impl<'t> PagesRequestBuilder<'t, PageInfo> {
//     /// Info for a given Page
//     ///
//     /// # Example
//     /// ```no_run
//     /// # use domo_pitchfork::error::PitchforkError;
//     /// use domo_pitchfork::pitchfork::DomoPitchfork;
//     /// let domo = DomoPitchfork::with_token("token");
//     /// let page_id = 123; // id of page to get details for.
//     /// let ds_info = domo.pages().info(page_id);
//     /// match ds_info {
//     ///     Ok(ds) => println!("{:?}",ds),
//     ///     Err(e) => println!("{}", e)
//     /// };
//     /// ```
//     pub fn info(mut self, page_id: u64) -> Result<PageInfo, PitchforkError> {
//         self.url.push_str(&page_id.to_string());
//         let req = Self {
//             method: Method::GET,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         };
//         req.retrieve_and_deserialize_json()
//     }

//     /// List Pages starting from a given offset up to a given limit.
//     /// # Example
//     /// ```no_run
//     /// # use domo_pitchfork::error::PitchforkError;
//     /// use domo_pitchfork::pitchfork::DomoPitchfork;
//     /// let domo = DomoPitchfork::with_token("token");
//     /// let list = domo.pages().list(5,0)?;
//     /// list.iter().map(|p| println!("Page Name: {}", p.name));
//     /// # Ok::<(),PitchforkError>(())
//     /// ```
//     pub fn list(mut self, limit: u32, offset: u32) -> Result<Vec<PageInfo>, PitchforkError> {
//         self.url
//             .push_str(&format!("?limit={}&offset={}", limit, offset));
//         let req = Self {
//             method: Method::GET,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         };
//         let ds_list = serde_json::from_reader(req.send_json()?)?;
//         Ok(ds_list)
//     }

//     pub fn create(self, page: &PageInfo) -> Result<PageInfo, PitchforkError> {
//         let body = serde_json::to_string(page)?;
//         debug!("body: {}", body);
//         let req = Self {
//             method: Method::POST,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: Some(body),
//         };
//         req.retrieve_and_deserialize_json()
//     }

//     /// Delete the Page for the given id.
//     /// This is destructive and cannot be reversed.
//     /// # Example
//     /// ```no_run
//     /// # use domo_pitchfork::error::PitchforkError;
//     /// # use domo_pitchfork::pitchfork::DomoPitchfork;
//     /// # let token = "token_here";
//     /// let domo = DomoPitchfork::with_token(&token);
//     ///
//     /// let page_id = 123; // id of page to delete.
//     /// // if it fails to delete, print err msg
//     /// if let Err(e) = domo.pages().delete(page_id) {
//     ///     println!("{}", e)
//     /// }
//     /// ```
//     pub fn delete(mut self, page_id: u64) -> Result<(), PitchforkError> {
//         self.url.push_str(&page_id.to_string());
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

//     pub fn modify(mut self, page_id: u64, page: &PageInfo) -> Result<PageInfo, PitchforkError> {
//         self.url.push_str(&page_id.to_string());
//         let body = serde_json::to_string(page)?;
//         debug!("body: {}", body);
//         let req = Self {
//             method: Method::PUT,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: Some(body),
//         };
//         let ds = serde_json::from_reader(req.send_json()?)?;
//         Ok(ds)
//     }

//     pub fn collections(mut self, page_id: u64) -> Result<Vec<PageCollection>, PitchforkError> {
//         self.url.push_str(&format!("{}/collections", page_id));
//         let req = Self {
//             method: Method::GET,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: None,
//         };
//         let ds = serde_json::from_reader(req.send_json()?)?;
//         Ok(ds)
//     }
//     // TODO: check domo does in fact return an empty body here
//     pub fn create_collection(
//         mut self,
//         page_id: u64,
//         collection: &PageCollection,
//     ) -> Result<(), PitchforkError> {
//         self.url.push_str(&format!("{}/collections", page_id));
//         let body = serde_json::to_string(collection)?;
//         debug!("body: {}", body);
//         let req = Self {
//             method: Method::POST,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: Some(body),
//         };
//         let ds = serde_json::from_reader(req.send_json()?)?;
//         Ok(ds)
//     }
//     pub fn modify_collection(
//         mut self,
//         page_id: u64,
//         collection_id: u64,
//         collection: &PageCollection,
//     ) -> Result<(), PitchforkError> {
//         self.url
//             .push_str(&format!("{}/collections/{}", page_id, collection_id));
//         let body = serde_json::to_string(collection)?;
//         debug!("body: {}", body);
//         let req = Self {
//             method: Method::PUT,
//             auth: self.auth,
//             url: self.url,
//             resp_t: PhantomData,
//             body: Some(body),
//         };
//         req.send_json()?;
//         Ok(())
//     }
//     pub fn delete_collection(
//         mut self,
//         page_id: u64,
//         collection_id: u64,
//     ) -> Result<(), PitchforkError> {
//         self.url
//             .push_str(&format!("{}/collections/{}", page_id, collection_id));
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
// }
