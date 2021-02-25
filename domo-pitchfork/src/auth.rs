//! Authorization/Authentication for Domo API.
//!


// /// `OAuth` authorization scopes for the Domo API
// #[derive(Default)]
// pub struct DomoScope {
//     pub data: bool,
//     pub user: bool,
//     pub audit: bool,
//     pub dashboard: bool,
//     pub buzz: bool,
//     pub account: bool,
//     pub workflow: bool,
// }

//     fn request_access_token(&self) -> Option<DomoToken> {
//         let mut payload = HashMap::new();
//         payload.insert("grant_type", "client_credentials");
//         let mut scopes = "".to_string();
//         if self.domo_scope.data {
//             if !scopes.is_empty() {
//                 scopes += &"%20".to_string()
//             }
//             scopes += &"data".to_string();
//         }
//         if self.domo_scope.user {
//             if !scopes.is_empty() {
//                 scopes += &"%20".to_string()
//             }
//             scopes += &"user".to_string();
//         }
//         if self.domo_scope.audit {
//             if !scopes.is_empty() {
//                 scopes += &"%20".to_string()
//             }
//             scopes += &"audit".to_string();
//         }
//         if self.domo_scope.dashboard {
//             if !scopes.is_empty() {
//                 scopes += &"%20".to_string()
//             }
//             scopes += &"dashboard".to_string();
//         }

//         if let Some(token) = self.fetch_access_token(&self.client_id, &self.client_secret, &scopes)
//         {
//             Some(token)
//         } else {
//             None
//         }
//     }

use serde::{Deserialize, Serialize};
use std::sync::{Mutex, Arc};
use log::debug;

#[derive(Clone, Debug)]
pub struct DomoAuth {
    pub token: DomoToken,
    pub time_acquired: std::time::Instant,
    pub token_refresh_count: usize,
    pub token_use_count: usize,
}
/// Domo oauth2 token
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomoToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
    pub customer: String,
    pub env: String,
    #[serde(rename = "userId")]
    pub user_id: u32,
    pub role: String,
    pub jti: String,
    pub domain: String,
}

impl DomoToken {
    #[must_use]
    pub fn default() -> Self {
        Self {
            access_token: String::new(),
            token_type: String::new(),
            expires_in: 0,
            scope: String::new(),
            customer: String::new(),
            env: String::new(),
            jti: String::new(),
            user_id: 0_u32,
            role: String::new(),
            domain: String::new(),
        }
    }

    #[must_use]
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = access_token.to_string();
        self
    }

    #[must_use]
    pub fn token_type(mut self, token_type: &str) -> Self {
        self.token_type = token_type.to_string();
        self
    }

    #[must_use]
    pub fn expires_in(mut self, expires_in: u64) -> Self {
        self.expires_in = expires_in;
        self
    }

    #[must_use]
    pub fn scope(mut self, scope: &str) -> Self {
        self.scope = scope.to_string();
        self
    }
}

#[derive(Debug)]
pub struct DomoAuthClient {
    client_id: String,
    secret: String,
    inner: Arc<Mutex<Option<DomoAuth>>>,
    client: reqwest::Client,
}

impl DomoAuthClient {
    pub fn new<S: Into<String>>(client_id: S, secret: S) -> Self {
        Self {
            client_id: client_id.into(),
            secret: secret.into(),
            inner: Arc::new(Mutex::new(None)),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_token(&self, ) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>  {
        let mut lock = self.inner.lock().unwrap();
        let is_expired = match lock.as_ref() {
            Some(a) => {
                a.time_acquired.elapsed().as_secs() > a.token.expires_in
            },
            None => true,
        };

        if is_expired {
            // TODO: auth scopes
            let uri = "https://api.domo.com/oauth/token?grant_type=client_credentials&scope=data";

            let client = reqwest::Client::new();
            let req = client
                .post(uri)
                .basic_auth(self.client_id.clone(), Some(self.secret.clone()))
                .send().await?
                .error_for_status()?;
            
            let token: DomoToken = req.json().await?;
            let access_token = token.access_token.clone();
            let refresh_cnt = if let Some(domo_auth) = lock.as_ref() {
                domo_auth.token_refresh_count + 1
            } else { 0 };

            debug!("refresh count: {}", refresh_cnt);
            let auth = DomoAuth {
                token,
                time_acquired: std::time::Instant::now(),
                token_refresh_count: refresh_cnt,
                token_use_count: 0,
            };
            let _ = lock.replace(auth);
            Ok(access_token)
        }
        else {
            if let Some(mut domo_auth) = lock.as_mut() {
                domo_auth.token_use_count += 1;
                debug!("use count: {}", domo_auth.token_use_count);
            }
            let access_token = lock.as_ref().unwrap().token.access_token.clone();
            Ok(access_token)
        }
    }
}
