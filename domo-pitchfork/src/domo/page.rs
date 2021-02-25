// use serde::{Deserialize, Serialize};

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