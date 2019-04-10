//! Domo Groups API
//! 
//! [Domo Groups API Reference](https://developer.domo.com/docs/groups-api-reference/groups)
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::pitchfork::{DomoRequest, GroupsRequestBuilder};
use crate::error::DomoError;
use log::debug;
use reqwest::Method;
use std::marker::PhantomData;

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
    pub id: String,
    pub name: Option<String>,
}

impl<'t> GroupsRequestBuilder<'t, GroupInfo> {
    /// Returns a group object if valid group ID was provided.
    /// When requesting, if the group ID is related to a customer
    /// that has been deleted, a subset of the group's information will be returned,
    /// including a deleted property, which will be true.
    /// 
    /// # Example
    /// ```no_run
    /// use rusty_pitchfork::domo_man::DomoManager;
    /// use rusty_pitchfork::domo_man::DomoRequest;
    /// let domo = DomoManager::with_token("token");
    /// let ds_info = domo.groups().info("group_id");
    /// match ds_info {
    ///     Ok(ds) => println!("{:?}",ds),
    ///     Err(e) => println!("{}", e)
    /// };
    /// ```
    pub fn info(mut self, group_id: u64) -> Result<GroupInfo, DomoError> {
        self.url.push_str(&group_id.to_string());
        let req = Self {
            method: Method::GET,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        req.retrieve_and_deserialize_json()
    }

    /// List Groups starting from a given offset up to a given limit.
    /// Max limit is 500.
    /// Offset is the offset of the group ID to begin list of groups within the response.
    /// # Example
    /// ```no_run
    /// use rusty_pitchfork::domo_man::DomoManager;
    /// use rusty_pitchfork::domo_man::DomoRequest;
    /// let domo = DomoManager::with_token("token");
    /// let list = domo.groups().list(5,0);
    /// match list {
    ///     Ok(ds) => println!("{:?}",ds),
    ///     Err(e) => println!("{}", e)
    /// };
    /// ```
    pub fn list(mut self, limit: u32, offset: u32) -> Result<Vec<GroupInfo>, DomoError> {
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

    /// Creates a new Domo Group.
    /// Returns a group object when successful. 
    /// The returned group will have user attributes based on the 
    /// information that was provided when group was created. 
    pub fn create(self, group: &GroupInfo) -> Result<GroupInfo, DomoError> {
        // TODO: check if name property is the only one accepted here. Domo docs make it appear that way.
        let body = serde_json::to_string(group)?;
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

    /// Delete the Group for the given id.
    /// This is destructive and cannot be reversed.
    /// # Example
    /// ```no_run
    /// # use rusty_pitchfork::domo_man::DomoManager;
    /// # use rusty_pitchfork::domo_man::DomoRequest;
    /// # let token = "token_here"
    /// let domo = DomoManager::with_token(&token);
    /// let d = domo.groups()
    ///             .delete("group_id");
    /// // if it fails to delete
    /// if let Err(e) = d {
    ///     println!("{}", e) 
    /// } 
    /// ```
    pub fn delete(mut self, group_id: u64) -> Result<(), DomoError> {
        self.url.push_str(&group_id.to_string());
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

    /// Modify an existing Domo Group.
    /// Updates the specified group by providing values to parameters passed.
    /// Any parameter left out of the request will cause the specific group’s attribute to remain unchanged.
    pub fn modify(
        mut self,
        group_id: u64,
        group: &GroupInfo,
    ) -> Result<(), DomoError> {
        // TODO: check if name and active are the only params accepted here
        // domo docs make it look that way
        self.url.push_str(&group_id.to_string());
        let body = serde_json::to_string(group)?;
        debug!("body: {}", body);
        let req = Self {
            method: Method::PUT,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(body),
        };
        let ds = serde_json::from_reader(req.send_json()?)?;
        Ok(ds)
    }

    /// Returns a list of user id's that are in a Group
    /// Limit is 500.
    /// 
    pub fn users(mut self, group_id: u64) -> Result<Vec<u64>, DomoError> {
        // TODO: add limit/offset params.
        // Domo docs indicates the max limit is 500
        self.url.push_str(&format!("{}/users/", group_id));
        let req = Self {
            method: Method::GET,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        };
        let ds = serde_json::from_reader(req.send_json()?)?;
        Ok(ds)
    }

    /// Adds a specified user to a group.
    pub fn add_user(mut self, group_id: u64, user_id: u64) -> Result<(), DomoError> {
        self.url.push_str(&format!("{}/users/{}", group_id, user_id));
        let req = Self {
            method: Method::PUT,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(json!({}).to_string()),
        };
        let res = req.send_json()?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(DomoError::Other(format!("HTTP Status: {}", res.status())))
        }
    }

    /// Removes a specified user from a specified Group.
    pub fn remove_user(mut self, group_id: u64, user_id: u64) -> Result<(), DomoError> {
        self.url.push_str(&format!("{}/users/{}", group_id, user_id));
        let req = Self {
            method: Method::DELETE,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(json!({}).to_string()),
        };
        let res = req.send_json()?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(DomoError::Other(format!("HTTP Status: {}", res.status())))
        }
    }
}