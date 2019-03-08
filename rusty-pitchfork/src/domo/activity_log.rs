use serde::{Deserialize, Serialize};

// [Activity Log Entry object](https://developer.domo.com/docs/activity-log-api-reference/activity-log)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivityLog {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userType")]
    pub user_type: String,
    #[serde(rename = "actorId")]
    pub actor_id: u64,
    #[serde(rename = "actorType")]
    pub actor_type: String,
    #[serde(rename = "objectName")]
    pub object_name: String,
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[serde(rename = "additionalComment")]
    pub additional_comment: String,
    pub time: String,
    #[serde(rename = "eventText")]
    pub event_text: String,
    pub device: String,
    #[serde(rename = "browserDetails")]
    pub browser_details: String,
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
}
