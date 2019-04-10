//! Domo Users API
//! 
//! [Domo Users API Reference](https://developer.domo.com/docs/users-api-reference/users)
use crate::domo::group::Group;
use serde::{Deserialize, Serialize};
use crate::pitchfork::{DomoRequest, UsersRequestBuilder};
use crate::error::DomoError;
use log::debug;
use reqwest::Method;
use std::marker::PhantomData;

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
impl<'t> UsersRequestBuilder<'t, User> {
    /// Returns a user object if valid user ID was provided.
    /// When requesting, if the user ID is related to a user that has been deleted,
    /// a subset of the user information will be returned,
    /// including a deleted property, which will be true.
    /// 
    /// # Example
    /// ```no_run
    /// use rusty_pitchfork::domo_man::DomoManager;
    /// use rusty_pitchfork::domo_man::DomoRequest;
    /// let domo = DomoManager::with_token("token");
    /// let ds_info = domo.users().info("user_id");
    /// match ds_info {
    ///     Ok(ds) => println!("{:?}",ds),
    ///     Err(e) => println!("{}", e)
    /// };
    /// ```
    pub fn info(mut self, user_id: u64) -> Result<User, DomoError> {
        self.url.push_str(&user_id.to_string());
        let req = Self {
            method: Method::GET,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        req.retrieve_and_deserialize_json()
    }

    /// List Users starting from a given offset up to a given limit.
    /// Max limit is 500.
    /// offset is the offset of the user ID to begin list of users within the response.
    /// # Example
    /// ```no_run
    /// use rusty_pitchfork::domo_man::DomoManager;
    /// use rusty_pitchfork::domo_man::DomoRequest;
    /// let domo = DomoManager::with_token("token");
    /// let list = domo.users().list(5,0);
    /// match list {
    ///     Ok(ds) => println!("{:?}",ds),
    ///     Err(e) => println!("{}", e)
    /// };
    /// ```
    pub fn list(mut self, limit: u32, offset: u32) -> Result<Vec<User>, DomoError> {
        self.url
            .push_str(&format!("?limit={}&offset={}", limit, offset));
        let req = Self {
            method: Method::GET,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        let ds_list = serde_json::from_reader(req.send_json()?)?;
        Ok(ds_list)
    }

    pub fn create(self, user: &User) -> Result<User, DomoError> {
        // TODO: validate that required fields: name, email, role were provided
        let body = serde_json::to_string(user)?;
        debug!("body: {}", body);
        let req = Self {
            method: Method::POST,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(body),
        };
        req.retrieve_and_deserialize_json()
    }

    /// Delete the User for the given id.
    /// This is destructive and cannot be reversed.
    /// # Example
    /// ```no_run
    /// # use rusty_pitchfork::domo_man::DomoManager;
    /// # use rusty_pitchfork::domo_man::DomoRequest;
    /// # let token = "token_here"
    /// let domo = DomoManager::with_token(&token);
    /// let d = domo.users()
    ///             .delete("user_id");
    /// // if it fails to delete
    /// if let Err(e) = d {
    ///     println!("{}", e) 
    /// } 
    /// ```
    pub fn delete(mut self, user_id: u64) -> Result<(), DomoError> {
        self.url.push_str(&user_id.to_string());
        let req = Self {
            method: Method::DELETE,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        let res = req.send_json()?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(DomoError::Other(format!("HTTP Status: {}", res.status())))
        }
    }

    /// Update an existing user.
    /// Known Limitation: as of 4/10/19 all user fields are required by the Domo API
    pub fn modify(
        mut self,
        user_id: u64,
        user: &User,
    ) -> Result<(), DomoError> {
        self.url.push_str(&user_id.to_string());
        let body = serde_json::to_string(user)?;
        debug!("body: {}", body);
        let req = Self {
            method: Method::PUT,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(body),
        };
        let res = req.send_json()?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(DomoError::Other(format!("HTTP Status: {}", res.status())))
        }
    }
}