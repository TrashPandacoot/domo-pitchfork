use serde::{Deserialize, Serialize};

// [Group Object](https://developer.domo.com/docs/groups-api-reference/groups-2
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupInfo {
    pub name: String,
    pub default: bool,
    pub active: bool,
    pub creator_id: String,
    pub member_count: i32,
    pub user_ids: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: Option<String>,
}
