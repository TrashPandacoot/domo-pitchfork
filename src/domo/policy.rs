use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    pub id: usize,
    pub name: String,
    #[serde(rename = "type")]
    pub policy_type: String,
    #[serde(rename = "users")]
    pub user_ids: Vec<usize>,
    #[serde(rename = "groups")]
    pub group_ids: Vec<usize>,
    pub filters: Vec<Filter>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    pub column: String,
    pub not: bool,
    pub operator: String,
    pub values: Vec<String>,
}
