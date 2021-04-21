use serde::{Deserialize, Serialize};
// [Activity Log Entry object](https://developer.domo.com/docs/activity-log-api-reference/activity-log)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivityLogEntry {
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

#[derive(Debug)]
pub struct ActivityLogSearchQuery {
    pub user_id: Option<u64>,
    /// The start time(milliseconds) of when you want to receive log events
    pub start: u64,
    /// The end time(milliseconds) of when you want to receive log events
    pub end: Option<u64>,
    /// The maximum number of events you want to retrieve. Default is 50, maximum is 1000.
    pub limit: Option<u32>,
    /// The offset location of events you retrieve. Default is 0.
    pub offset: Option<u32>,
}
impl ActivityLogSearchQuery {
    #[allow(dead_code)]
    pub(crate) fn create_query_string(&mut self) -> String {
        let mut s = String::new();
        s.push_str("start=");
        s.push_str(&self.start.to_string());
        if self.end.is_some() {
            s.push_str("&end=");
            s.push_str(self.end.take().unwrap().to_string().as_ref());
        }
        if self.limit.is_some() {
            s.push_str("&limit=");
            s.push_str(self.limit.take().unwrap().to_string().as_ref());
        }
        if self.offset.is_some() {
            s.push_str("&offset=");
            s.push_str(self.offset.take().unwrap().to_string().as_ref());
        }
        if self.user_id.is_some() {
            s.push_str("&user=");
            s.push_str(self.user_id.take().unwrap().to_string().as_ref());
        }
        s
    }
}