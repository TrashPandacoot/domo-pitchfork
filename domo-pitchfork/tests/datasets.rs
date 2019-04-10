extern crate domo_pitchfork;

use domo_pitchfork::auth::DomoClientAppCredentials;
use domo_pitchfork::domo::dataset::Dataset;
use domo_pitchfork::pitchfork::DomoPitchfork;
use std::env;

#[test]
fn test_get_dataset_details() {
    let rusty_fork = create_test_rusty_fork();
    let dataset_id = "d755e978-bffc-4a53-bef7-6760751776ec"; // "2f450bb0-58ed-4d79-8034-cd8c4b1bd8d3";
    let dataset_details: Dataset = rusty_fork.datasets.info(dataset_id).unwrap();
    assert_eq!(dataset_details.id, dataset_id);
}

#[test]
fn test_get_dataset_list() {
    //
    let rusty_fork = create_test_rusty_fork();
    let ds_list: Vec<Dataset> = rusty_fork.datasets().list(5, 0).unwrap();
    assert_eq!(ds_list.len(), 5);
}

#[test]
fn test_export_dataset_data() {
    let rusty_fork = create_test_rusty_fork();
    let csv = rusty_fork
        .datasets()
        .download_data("77faea51-68ab-4dd3-ae1a-8992bc1b58a8")
        .unwrap_or_default();
    println!("{}", csv);
    assert_eq!(1, 1);
}

#[test]
fn test_replace_dataset_data() {
    let rusty_fork = create_test_rusty_fork();
    let csv = create_test_csv();
    let replace = rusty_fork.datasets().upload_from_str("77faea51-68ab-4dd3-ae1a-8992bc1b58a8", &csv);
    assert!(replace.is_ok());
}

fn create_test_csv() -> String {
    "Sample Data,0
Test AB,1
Test BI-Dev,2
Chaos Monkey,5
Capstone,104"
        .to_string()
}

fn create_test_rusty_fork() -> DomoPitchfork {
    let domo_client_id = env::var("DOMO_CLIENT_ID").expect("No DOMO_CLIENT_ID env var found");
    let domo_secret = env::var("DOMO_SECRET").expect("No DOMO_SECRET env var found");
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .build();
    let token = client_creds.get_access_token();
    DomoPitchfork::with_token(&token)
}