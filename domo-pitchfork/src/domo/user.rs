// //! Domo Users API
// //!
// //! # [`UsersRequestBuilder`](`crate::pitchfork::UsersRequestBuilder`) implements all available user API endpoints and functionality
// //!
// //! Additional Resources:
// //! - [Domo Users API Reference](https://developer.domo.com/docs/users-api-reference/users)
use crate::domo::group::Group;
use serde::{Deserialize, Serialize};

// [User Object](https://developer.domo.com/docs/users-api-reference/users-2)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u32>,
    pub title: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "alternateEmail")]
    pub alternate_email: Option<String>,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub name: Option<String>,
    pub location: Option<String>,
    pub timezone: Option<String>, // Might not show up in json unless it's been changed from the instance default tz
    #[serde(rename = "employeeId")]
    pub employee_id: Option<String>, // key doesn't show up in json if Null
    #[serde(rename = "roleId")]
    pub role_id: Option<usize>,
    #[serde(rename = "employeeNumber")]
    pub employee_number: Option<usize>, // Doesn't show up in JSON if it's null
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>, //TODO: make this a datetime
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>, //TODO: make this a datetime
    pub deleted: Option<bool>,
    #[serde(rename = "image")]
    pub image_uri: Option<String>,
    pub groups: Option<Vec<Group>>,
    pub locale: Option<String>,
}

// TODO: is the 'Editor' role not available to the api our is the documentation wrong?
pub enum Role {
    Participant,
    Privileged,
    Admin,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Owner {
    pub id: u32,
    pub name: String,
}