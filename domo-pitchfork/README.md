# Domo Pitchfork
[![crates.io](https://img.shields.io/crates/v/domo_pitchfork.svg)](https://crates.io/crates/domo_pitchfork)
[![Documentation](https://docs.rs/domo_pitchfork/badge.svg)](https://docs.rs/domo_pitchfork/1.4.1/domo_pitchfork/)

Domo Pitchfork is a rust lib crate for interacting with Domo's Public API. This lib is what powers the ripdomo CLI tool. 

# Pre-release
v1.5.0-alpha.0 is a work in progress migration to the surf http client library. Not all API endpoints have been implemented yet.
The data APIs will be prioritized in implementing first.

- [Changelog](changelog.md)

## Example
```rust,no_run
 use domo_pitchfork::auth::DomoClientAppCredentials;
 use domo_pitchfork::DomoPitchfork;
 use std::error::Error;

 fn main() -> Result<(), Box<dyn Error>> {
    let auth = DomoClientAppCredentials::default()
        .client_id("domo client ID here")
        .client_secret("domo secret here")
        .build();
    let token = auth.get_access_token();
    let domo = DomoPitchfork::with_token(&token);

    let dataset_list = domo.datasets().list(5,0)?;

    dataset_list.iter()
        .map(|ds| println!("Dataset Name: {}", ds.name.as_ref().unwrap()));
    Ok(())
}
```
