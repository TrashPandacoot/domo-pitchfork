//! Authorization/Authentication for Domo API.
//!
use crate::PitchforkError;
use log::{debug, error, trace};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

/// Domo auth token
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomoToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
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
    pub fn default() -> Self {
        Self {
            access_token: String::new(),
            token_type: String::new(),
            expires_in: 0_u32,
            scope: String::new(),
            customer: String::new(),
            env: String::new(),
            jti: String::new(),
            user_id: 0_u32,
            role: String::new(),
            domain: String::new(),
        }
    }

    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = access_token.to_string();
        self
    }

    pub fn token_type(mut self, token_type: &str) -> Self {
        self.token_type = token_type.to_string();
        self
    }

    pub fn expires_in(mut self, expires_in: u32) -> Self {
        self.expires_in = expires_in;
        self
    }

    pub fn scope(mut self, scope: &str) -> Self {
        self.scope = scope.to_string();
        self
    }
}
/// `OAuth` authorization scopes for the Domo API
#[derive(Default)]
pub struct DomoAuthScope {
    pub data: bool,
    pub user: bool,
    pub audit: bool,
    pub dashboard: bool,
    pub buzz: bool,
    pub account: bool,
    pub workflow: bool,
}

/// Info to store successful authentication token and keep track of expiration.
pub struct DomoAuth {
    pub domo_token: DomoToken,
    pub token_retrieved_at: std::time::Instant,
}

impl DomoAuth {
    pub fn new(domo_token: DomoToken) -> Self {
        Self {
            domo_token,
            token_retrieved_at: std::time::Instant::now(),
        }
    }
    /// Checks if the Authentication Token has expired.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.token_retrieved_at.elapsed().as_secs() > u64::from(self.domo_token.expires_in)
    }
}
/// Object to use to store/retrieve access tokens for Domo API.
pub struct DomoClientAuth {
    pub client_id: String,
    pub client_secret: String,
    pub(crate) auth: Arc<Mutex<Option<DomoAuth>>>,
    pub domo_scope: DomoAuthScope,
}

impl DomoClientAuth {
    pub fn is_expired(&self) -> bool {
        if let Some(d) = self.auth.lock().unwrap().as_ref() {
            dbg!(d.is_expired())
        } else {
            dbg!(true)
        }
    }
    /// Default initialization attempts to populate client_id from env::var("DOMO_CLIENT_ID") and populate client_secret from env::var("DOMO_SECRET")
    #[must_use]
    pub fn default() -> Self {
        let client_id = env::var("DOMO_CLIENT_ID").unwrap_or_default();
        let client_secret = env::var("DOMO_SECRET").unwrap_or_default();
        let data_scope: bool = env::var("DATA_SCOPE").is_ok();
        let user_scope: bool = env::var("USER_SCOPE").is_ok();
        let audit_scope: bool = env::var("AUDIT_SCOPE").is_ok();
        let dashboard_scope: bool = env::var("DASHBOARD_SCOPE").is_ok();
        if data_scope && user_scope && audit_scope && dashboard_scope {
            let scope = DomoAuthScope {
                data: data_scope,
                user: user_scope,
                audit: audit_scope,
                dashboard: dashboard_scope,
                buzz: false,
                account: false,
                workflow: false,
            };
            Self {
                client_id,
                client_secret,
                auth: Arc::new(Mutex::new(None)),
                domo_scope: scope,
            }
        } else {
            let scope = DomoAuthScope {
                data: true,
                user: false,
                audit: false,
                dashboard: false,
                buzz: false,
                account: false,
                workflow: false,
            };
            Self {
                client_id,
                client_secret,
                auth: Arc::new(Mutex::new(None)),
                domo_scope: scope,
            }
        }
    }

    /// Returns an Option reference to a valid OAuth2 access token if available.
    #[must_use]
    pub fn bearer_token(&self) -> Option<String> {
        if let Some(token) = self.auth.lock().unwrap().as_ref() {
            Some(token.domo_token.access_token.to_string())
        } else {
            None
        }
    }
    pub fn add_data_scope(&mut self) {
        self.domo_scope.data = true;
    }

    // pub async fn auth(&self) -> Result<(), PitchforkError> {
    //     if self.is_expired() {
    //         trace!("refreshing Domo authentication");
    //         dbg!("ref Domo auth");
    //         self.domo_oauth2_login().await?;
    //     } else {
    //         dbg!("already auth");
    //         trace!("Already Authenticated");
    //     }
    //     Ok(())
    // }
    // Check if authenticated with Domo. If there's an existing Token check if it's still valid.
    // If there's no existing Token, or if the existing one has expired, re-authenticate with Domo to get a fresh DomoToken
    pub async fn authenticate(&self) -> Result<(), PitchforkError> {
        let mut exp = false;
        if let Some(a) = self.auth.clone().lock().unwrap().as_ref() {
            if a.is_expired() {
                dbg!("refreshing Domo authentication");
                // self.domo_oauth2_login().await?;
                exp = true;
            } else {
                trace!("Already Authenticated");
            }
        // Ok(())
        } else {
            trace!("performing initial Domo authentication");
            exp = true;
            // self.domo_oauth2_login().await?;
            // Ok(())
        }
        if exp {
            dbg!("refreshing Domo authentication");
            self.domo_oauth2_login().await?;
        }
        Ok(())
    }

    // Login/OAuth2 Authentication with Domo to retrieve fresh Auth Token.
    async fn domo_oauth2_login(&self) -> Result<(), PitchforkError> {
        let mut payload = HashMap::new();
        payload.insert("grant_type", "client_credentials");
        let mut scopes = "".to_string();
        if self.domo_scope.data {
            if !scopes.is_empty() {
                scopes += &"%20".to_string()
            }
            scopes += &"data".to_string();
        }
        if self.domo_scope.user {
            if !scopes.is_empty() {
                scopes += &"%20".to_string()
            }
            scopes += &"user".to_string();
        }
        if self.domo_scope.audit {
            if !scopes.is_empty() {
                scopes += &"%20".to_string()
            }
            scopes += &"audit".to_string();
        }
        if self.domo_scope.dashboard {
            if !scopes.is_empty() {
                scopes += &"%20".to_string()
            }
            scopes += &"dashboard".to_string();
        }
        dbg!(&scopes);
        // if self.client_id.is_empty() {
        //     self.set_client_id_from_env()?;
        // }
        // if self.client_secret.is_empty() {
        //     self.set_client_secret_from_env()?;
        // }
        if let Ok(token) = self
            .fetch_access_token(&self.client_id, &self.client_secret, &scopes)
            .await
        {
            dbg!("domo login token");
            self.auth
                .clone()
                .try_lock()
                .unwrap()
                .replace(DomoAuth::new(token));
            dbg!("domo login token replace");
            Ok(())
        } else {
            dbg!("domo login fetch_access_token failed");
            self.auth.lock().unwrap().take();
            Err(PitchforkError::from(
                "Failed OAuth2 authentication with Domo",
            ))
        }
    }

    pub fn client_id(mut self, client_id: &str) -> Self {
        self.client_id = client_id.to_string();
        self
    }
    pub fn set_client_id_from_env(&mut self) -> Result<(), PitchforkError> {
        self.client_id = std::env::var("DOMO_CLIENT_ID").map_err(PitchforkError::from)?;
        Ok(())
    }
    pub fn set_client_secret_from_env(&mut self) -> Result<(), PitchforkError> {
        self.client_secret = std::env::var("DOMO_SECRET").map_err(PitchforkError::from)?;
        Ok(())
    }

    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = client_secret.to_string();
        self
    }

    pub fn client_scope(mut self, domo_scope: DomoAuthScope) -> Self {
        self.domo_scope = domo_scope;
        self
    }

    pub fn with_user_scope(mut self) -> Self {
        self.domo_scope.user = true;
        self
    }

    pub fn with_data_scope(mut self) -> Self {
        self.domo_scope.data = true;
        self
    }
    pub fn with_audit_scope(mut self) -> Self {
        self.domo_scope.audit = true;
        self
    }
    pub fn with_dashboard_scope(mut self) -> Self {
        self.domo_scope.dashboard = true;
        self
    }
    pub fn with_buzz_scope(mut self) -> Self {
        self.domo_scope.buzz = true;
        self
    }
    pub fn with_workflow_scope(mut self) -> Self {
        self.domo_scope.workflow = true;
        self
    }
    pub fn with_account_scope(mut self) -> Self {
        self.domo_scope.account = true;
        self
    }

    pub fn build(self) -> Self {
        const ERROR_MESSAGE: &str = "Set your Domo API Credentials. You can do this by setting environment variables in `.env` file:
        CLIENT_ID='domo-client-id'
        CLIENT_SECRET='domo-client-secret'";

        let empty_flag = if self.client_id.is_empty() {
            true
        } else {
            self.client_secret.is_empty()
        };

        if empty_flag {
            eprintln!("{}", ERROR_MESSAGE);
        } else {
            //debug!("client_id and client_secret found");
        }
        self
    }

    async fn fetch_access_token(
        &self,
        client_id: &str,
        client_secret: &str,
        params: &str,
    ) -> Result<DomoToken, PitchforkError> {
        fetch_access_token(client_id, client_secret, params).await
    }
}

// OAuth2 Authentication Login
async fn fetch_access_token(
    client_id: &str,
    client_secret: &str,
    params: &str,
) -> Result<DomoToken, PitchforkError> {
    dbg!("fetch_access_token");
    let client = Client::new();
    let url: Cow<'_, str> = [
        "https://api.domo.com/oauth/token?grant_type=client_credentials&scope=",
        params,
    ]
    .concat()
    .into();
    dbg!(client_id, client_secret);
    let response = client
        .post(&url.into_owned())
        .basic_auth(client_id, Some(client_secret))
        .send()
        .await?;
    dbg!("fetch_access_token resp");
    if response.status().is_success() {
        let buf = response.text().await?;
        let token: DomoToken = serde_json::from_str(&buf).unwrap();
        dbg!("fetch_access_token success");
        Ok(token)
    } else if let Ok(body) = response.text().await {
        error!("Failed getting Domo Auth Token: {}", body);
        Err(PitchforkError::from(format!(
            "Failed getting Domo Auth Token: {}",
            body
        )))
    } else {
        error!("Failed getting Domo Auth Token");
        Err(PitchforkError::from("Failed getting Domo Auth Token"))
    }
}
