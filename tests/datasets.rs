extern crate domo_pitchfork;

use domo_pitchfork::auth::DomoClientAppCredentials;
use domo_pitchfork::domo::dataset::Dataset;
use domo_pitchfork::DomoPitchfork;
use domo_pitchfork::PitchforkErrorKind;
use std::env;

#[test]
fn test_get_dataset_details() {
    let token = get_domo_token();
    let rusty_fork = DomoPitchfork::with_token(&token);
    let dataset_id = "d755e978-bffc-4a53-bef7-6760751776ec"; // "2f450bb0-58ed-4d79-8034-cd8c4b1bd8d3";
    let dataset_details: Dataset = rusty_fork.datasets().info(dataset_id).unwrap();
    assert_eq!(dataset_details.id, dataset_id);
}

#[test]
fn test_get_dataset_list() {
    let token = get_domo_token();
    let rusty_fork = DomoPitchfork::with_token(&token);
    let ds_list: Vec<Dataset> = rusty_fork.datasets().list(5, 0).unwrap();
    assert_eq!(ds_list.len(), 5);
}

#[test]
fn test_dataset_data_query() {
    let token = get_domo_token();
    let domo = DomoPitchfork::with_token(&token);
    let sql_query = "SELECT * FROM table";
    let ds_id = "447a2858-9c1c-42a9-b90b-a5340268d90e";
    let data_query = domo.datasets().query_data(ds_id, sql_query);
    match data_query {
        Ok(data) => assert_ne!(data.num_columns, 0),
        Err(e) => panic!("{:#?}", e),
    }
}

#[test]
fn test_bad_column_dataset_data_query() {
    let token = get_domo_token();
    let domo = DomoPitchfork::with_token(&token);
    let sql_query = "SELECT `BAD COLUMN NAME` FROM table";
    let ds_id = "447a2858-9c1c-42a9-b90b-a5340268d90e";
    let data_query = domo.datasets().query_data(ds_id, sql_query);
    match data_query {
        Ok(_) => panic!("expected a PitchforkError result not an Ok result"),
        Err(e) => match e.kind {
            PitchforkErrorKind::DomoBadRequest(c, e) => {
                assert_eq!(400, c);
                assert_eq!(
                    "There was a problem executing the SQL query: Invalid column(s) referenced",
                    e
                );
            }
            _ => panic!("expected a different PitchforkError type"),
        },
    };
}

#[test]
fn test_bad_sql_dataset_data_query() {
    let token = get_domo_token();
    let domo = DomoPitchfork::with_token(&token);
    let sql_query = "SELECT * FROM tablz WHERE ";
    let ds_id = "447a2858-9c1c-42a9-b90b-a5340268d90e";
    let data_query = domo.datasets().query_data(ds_id, sql_query);
    match data_query {
        Ok(_) => panic!("expected a PitchforkError result not an Ok result"),
        Err(e) => match e.kind {
            PitchforkErrorKind::DomoBadRequest(c, e) => {
                assert_eq!(400, c);
                assert_eq!(
                    "There was a problem executing the SQL query: Error processing query request: Unable to parse SQL: SELECT * FROM tablz WHERE ",
                    e
                );
            }
            _ => panic!("expected a different PitchforkError type"),
        },
    };
}

#[test]
fn test_export_dataset_data() {
    let token = get_domo_token();
    let rusty_fork = DomoPitchfork::with_token(&token);
    let csv = rusty_fork
        .datasets()
        .download_data("77faea51-68ab-4dd3-ae1a-8992bc1b58a8", false);
    match csv {
        Ok(s) => assert_eq!(create_test_csv(), s),
        Err(e) => panic!("{:#?}", e),
    };
}

#[test]
fn test_replace_dataset_data() {
    let token = get_domo_token();
    let rusty_fork = DomoPitchfork::with_token(&token);
    let csv = create_test_csv();
    let replace = rusty_fork
        .datasets()
        .upload_from_str("77faea51-68ab-4dd3-ae1a-8992bc1b58a8", csv);
    assert!(replace.is_ok());
}

fn create_test_csv() -> String {
    "Sample Data,0
Test AB,1
Test BI-Dev,2
Chaos Monkey,5
Capstone,104
"
    .to_string()
}

fn get_domo_token() -> String {
    let domo_client_id = env::var("DOMO_CLIENT_ID").expect("No DOMO_CLIENT_ID env var found");
    let domo_secret = env::var("DOMO_SECRET").expect("No DOMO_SECRET env var found");
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .build();
    client_creds.get_access_token()
}
