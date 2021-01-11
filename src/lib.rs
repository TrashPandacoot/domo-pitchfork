//! # Domo Pitchfork Domo SDK
//!
//! A Library wrapping the [Domo API](https://developer.domo.com) providing convenient consumption
//! of Domo Endpoints from Rust Programs. Why is it called Domo Pitchfork? Well for awhile Domo's API
//! documentation was pretty error ridden leading to joking around the office that anytime I was headed
//! to use the Domo API it was time to "bring out the pitchforks". The other reason is it ends up being
//! the pitchfork to move heaps of data in and out of Domo.
//!
//! # Example: Getting a list of Datasets
//! ```no_run
//! # use domo_pitchfork::auth::DomoClientAppCredentials;
//! # use domo_pitchfork::pitchfork::DomoPitchfork;
//! # use domo_pitchfork::error::PitchforkError;
//! let auth = DomoClientAppCredentials::default()
//!     .client_id("domo client ID here")
//!     .client_secret("domo secret here")
//!     .build();
//! let token = auth.get_access_token();
//! let domo = DomoPitchfork::with_token(&token);
//! let dataset_list = domo.datasets().list(5,0)?;
//! dataset_list.iter().map(|ds| println!("Dataset Name: {}", ds.name.as_ref().unwrap()));
//! # Ok::<(), PitchforkError>(())
//! ```
//!
//! ## [**`DomoPitchfork`**](pitchfork/index.html)
//!
//! The main module to be used and consumed by Rust Programs. The `DomoPitchfork`
//! struct has all the methods implemented to authenticate, interact, and consume the Domo API
//!
#![warn(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]

use std::sync::Arc;

use auth::DomoAuthClient;
#[cfg(test)]
use doc_comment::doctest;
use domo::data::{datasets::DatasetApiBuilder, streams::StreamBuilder};

#[cfg(test)]
doctest!("../README.md");

#[doc(inline)]
pub use self::error::{PitchforkError, PitchforkErrorKind};
// #[doc(inline)]
// pub use self::pitchfork::DomoPitchfork;

/// Authentication functionality for interacting with Domo API.
pub mod auth;
/// Domo API Types
pub mod domo;
/// Domo API errors
pub mod error;
/// Main Domo API Client.
pub mod pitchfork;
/// Generic Utility Functions.
pub mod util;

pub use crate::domo::data::stream_upload;

#[derive(Clone)]
pub struct DomoClient {
    inner: Arc<DomoApi>
}

impl  DomoClient {
    pub fn new<S: Into<String>>(client_id: S, secret: S) -> Self {
        Self {
            inner: Arc::new(DomoApi::new(client_id, secret))
        }
    }
    pub fn streams(&self) -> StreamBuilder {
        StreamBuilder {
            client: self.inner.clone()
        }
    }
    pub fn datasets(&self) -> DatasetApiBuilder {
        DatasetApiBuilder {
            client: self.inner.clone()
        }
    }
}

pub struct DomoApi {
    auth: DomoAuthClient,
    client: reqwest::Client,
}

impl DomoApi {
    pub fn new<S: Into<String>>(domo_client_id: S, domo_secret: S) -> Self {
        Self {
            client: reqwest::Client::new(),
            auth: DomoAuthClient::new(domo_client_id, domo_secret),
        }
    }
}
