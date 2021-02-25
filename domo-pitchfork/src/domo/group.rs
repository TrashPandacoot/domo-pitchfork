//! Domo Groups API
//!
//! # [`GroupsRequestBuilder`](`crate::pitchfork::GroupsRequestBuilder`) implements all available group API endpoints and functionality
//!
//! Additional Resources:
//! - [Domo Groups API Reference](https://developer.domo.com/docs/groups-api-reference/groups)
use serde::{Deserialize, Serialize};

// [Group Object](https://developer.domo.com/docs/groups-api-reference/groups-2
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupInfo {
    pub id: u64,
    pub name: String,
    pub default: bool,
    pub active: bool,
    #[serde(rename = "creatorId")]
    pub creator_id: u64,
    #[serde(rename = "memberCount")]
    pub member_count: i32,
    #[serde(rename = "userIds")]
    pub user_ids: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    //    pub id: String,
    pub id: u64, // User API started returning this as a number not a string. Found this 6/27/19.
    pub name: Option<String>,
}
