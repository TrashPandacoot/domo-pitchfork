# Domo Pitchfork
Domo Pitchfork is a rust lib crate for interacting with Domo's Public API. This lib is what powers the ripdomo CLI tool. 

- [Changelog](changelog.md)

## Example
```rust
 use domo_pitchfork::auth::DomoClientAppCredentials;
 use domo_pitchfork::pitchfork::DomoPitchfork;
 use domo_pitchfork::error::DomoError;

 fn main() -> Result<(), DomoError> {
    let auth = DomoClientAppCredentials::default()
        .client_id("domo client ID here")
        .client_secret("domo secret here")
        .build();
    let token = auth.get_access_token();
    let domo = DomoPitchfork::with_token(&token);

    let dataset_list = domo.datasets().list(5,0)?;

    dataset_list.iter()
        .map(|ds| println!("Dataset Name: {}", ds.name.as_ref().unwrap()));
}
```