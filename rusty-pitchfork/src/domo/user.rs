use crate::domo::group::Group;
use serde::{Deserialize, Serialize};

// [User Object](https://developer.domo.com/docs/users-api-reference/users-2)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub title: Option<String>,
    pub alternate_email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub timezone: Option<String>,
    pub image_uri: Option<String>,
    pub employee_number: Option<String>,
    pub groups: Option<Vec<Group>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Owner {
    pub id: u32,
    pub name: String,
}
