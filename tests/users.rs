// extern crate domo_pitchfork;

// use domo_pitchfork::auth::{DomoClientAppCredentials, DomoScope};
// use domo_pitchfork::domo::user::User;
// use domo_pitchfork::DomoPitchfork;
// use domo_pitchfork::PitchforkErrorKind;
// use std::env;

// #[test]
// fn test_get_user_details() {
//     let token = get_domo_token();
//     let rusty_fork = DomoPitchfork::with_token(&token);
//     let user_id = 1704739518u32; // Ryan Wilson user_id in instance
//     let user_details: User = rusty_fork.users().info(user_id as u64).unwrap();
//     assert_eq!(user_details.id, Some(user_id));
// }

// #[test]
// fn test_get_user_list() {
//     let token = get_domo_token();
//     let rusty_fork = DomoPitchfork::with_token(&token);
//     let users_list: Vec<User> = rusty_fork.users().list(5, 0).unwrap();
//     assert_eq!(users_list.len(), 5);
// }

// fn get_domo_token() -> String {
//     let domo_client_id = env::var("DOMO_CLIENT_ID").expect("No DOMO_CLIENT_ID env var found");
//     let domo_secret = env::var("DOMO_SECRET").expect("No DOMO_SECRET env var found");
//     let client_creds = DomoClientAppCredentials::default()
//         .client_id(&domo_client_id)
//         .client_secret(&domo_secret)
//         .with_user_scope()
//         .build();
//     client_creds.get_access_token()
// }
