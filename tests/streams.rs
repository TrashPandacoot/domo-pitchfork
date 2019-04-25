extern crate domo_pitchfork;
extern crate serde_json;

use domo_pitchfork::auth::DomoClientAppCredentials;
use domo_pitchfork::domo::dataset::{Column, DatasetSchema, Schema};
use domo_pitchfork::domo::stream::{StreamDatasetSchema, StreamSearchQuery};
use domo_pitchfork::pitchfork::DomoPitchfork;
use std::env;

#[test]
fn test_list_search_by_dataset_id_params_are_not_ignored_by_domo() {
    let token = get_domo_token();
    let domo = DomoPitchfork::with_token(&token);
    let first_five_no_search = domo.streams().list(5, 0).unwrap();
    let query = StreamSearchQuery::DatasetId("d47f9e01-9032-4201-a8c6-e2facc714de3".to_owned());
    let search = domo.streams().search(query).unwrap();
    dbg!(first_five_no_search[0].id);
    dbg!(search[0].id);
    assert_ne!(first_five_no_search[0].id, search[0].id);
}

#[test]
fn test_stream_e2e() {
    let token = get_domo_token();
    let domo = DomoPitchfork::with_token(&token);
    let csv = create_test_csv();
    let c = Column {
        column_type: "STRING".to_string(),
        name: "column name".to_string(),
    };
    let c2 = Column {
        column_type: "LONG".to_string(),
        name: "column name 2".to_string(),
    };
    let s = Schema {
        columns: vec![c, c2],
    };
    let ds = DatasetSchema {
        name: "Rusty Stream Test".to_string(),
        description: "Rusty Stream Test with compression".to_string(),
        rows: 0u32,
        schema: s,
    };
    let stream_ds = StreamDatasetSchema {
        dataset_schema: ds,
        update_method: "APPEND".to_string(),
    };

    //let new_stream_ds = NewStreamDataset {
    //dataset: stream_ds,
    //};

    //let v = serde_json::to_string_pretty(&new_stream_ds).unwrap();

    let stream = domo.streams().create(&stream_ds).unwrap();
    let e = domo.streams().create_stream_execution(stream.id).unwrap();
    domo.streams()
        .upload_part(stream.id, e.id, 1u32, &csv)
        .unwrap();
    let _commit = domo.streams().commit_execution(stream.id, e.id).unwrap();
    domo.streams().delete(stream.id).unwrap();
    assert_eq!(1, 1);
}

fn create_test_csv() -> String {
    "Sample Data,0
Test AB,1
Test BI-Dev,2
Chaos Monkey,5
Capstone,104"
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
