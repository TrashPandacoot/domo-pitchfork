//! # Ripdomo Rusty Pitchfork Library
//!
//! A Library wrapping the [Domo API](https://developer.domo.com) providing convenient consumption
//! of Domo Endpoints from Rust Programs
//!
//! ## [**Client**](client/index.html)
//!
//! The main public module to be used and consumed by Rust Programs. The RustyPitchfork
//! struct has all the methods implemented to authenticate, interact, and consume the Domo API
//!
//! ## [**Authentication**](authentication/index.html)
//!
//! Module implementing authentication with the Domo API. This OAuth2 impl is set to use domo client id and secrets
//! stored as environmental variables if not provided explicitly.
//!
//! [**Domo**](domo/index.html)
//!
//! Module containing all the structs for modeling the data structures needed for interacting with the Domo API
//! as well as the data that can be retreived from the API
#![warn(clippy::all, clippy::pedantic)]
extern crate csv;
extern crate reqwest;
// extern crate serde;
use serde::{Deserialize, Serialize};
#[macro_use]
extern crate serde_json;
extern crate chrono;

#[macro_use]
extern crate lazy_static;

/// Domo API errors
pub mod error;
/// Authentication functionality for interacting with Domo API.
pub mod auth;
/// Client Implementation. Main public module of the library.
pub mod client;
/// Domo API Types
pub mod domo;
/// Generic Utility Functions.
pub mod util;
