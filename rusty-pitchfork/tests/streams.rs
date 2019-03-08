extern crate dotenv;
extern crate rusty_pitchfork;
extern crate serde_json;

use rusty_pitchfork::domo::stream::StreamDatasetSchema;
use rusty_pitchfork::domo::dataset::{Column, DatasetSchema, Schema};
use rusty_pitchfork::auth::DomoClientAppCredentials;
use rusty_pitchfork::client::RustyPitchfork;
use std::collections::HashMap;
use std::env;

#[test]
fn test_list_limit_params_are_not_ignored_by_domo() {
    let domo = create_test_rusty_fork();
    let list_url = "v1/streams".to_owned();
    let mut lim_1_params = HashMap::new();
    lim_1_params.insert("limit".to_owned(), "1".to_owned());
    let mut lim_5_params = HashMap::new();
    lim_5_params.insert("limit".to_owned(), "5".to_owned());
    let lim_1 = domo.domo_get(&list_url, &lim_1_params).unwrap();
    let lim_5 = domo.domo_get(&list_url, &lim_5_params).unwrap();
    assert_ne!(lim_1, lim_5);
    let list_1 = domo.list_streams(1i32, 0i32).unwrap();
    let list_5 = domo.list_streams(5i32, 0i32).unwrap();
    assert_ne!(list_1.len(), list_5.len());
    assert_eq!(list_1.len(), 1);
    assert_eq!(list_5.len(), 5);
}

#[test]
fn test_list_offset_params_are_not_ignored_by_domo() {
    let domo = create_test_rusty_fork();
    let list_url = "v1/streams".to_owned();
    let mut offset_0_params = HashMap::new();
    offset_0_params.insert("limit".to_owned(), "3".to_owned());
    offset_0_params.insert("offset".to_owned(), "0".to_owned());
    let mut offset_2_params = HashMap::new();
    offset_2_params.insert("limit".to_owned(), "3".to_owned());
    offset_2_params.insert("offset".to_owned(), "2".to_owned());
    let off_0 = domo.domo_get(&list_url, &offset_0_params).unwrap();
    let off_2 = domo.domo_get(&list_url, &offset_2_params).unwrap();
    assert_ne!(off_0, off_2);
    let list_off_0 = domo.list_streams(3i32, 0i32).unwrap();
    let list_off_2 = domo.list_streams(3i32, 2i32).unwrap();
    assert_ne!(list_off_0[0].id, list_off_2[0].id);
    assert_eq!(list_off_0[2].id, list_off_2[0].id);
}

#[test]
fn test_list_sort_params_are_not_ignored_by_domo() {
    let domo = create_test_rusty_fork();
    let list_url = "v1/streams".to_owned();
    let mut sort_name_asc_params = HashMap::new();
    sort_name_asc_params.insert("limit".to_owned(), "5".to_owned());
    sort_name_asc_params.insert("offset".to_owned(), "0".to_owned());
    sort_name_asc_params.insert("sort".to_owned(), "name".to_owned());
    let mut sort_name_desc_params = HashMap::new();
    sort_name_desc_params.insert("limit".to_owned(), "5".to_owned());
    sort_name_desc_params.insert("offset".to_owned(), "0".to_owned());
    sort_name_desc_params.insert("sort".to_owned(), "-name".to_owned());
    let name_asc = domo.domo_get(&list_url, &sort_name_asc_params).unwrap();
    let name_desc = domo.domo_get(&list_url, &sort_name_desc_params).unwrap();
    assert_ne!(name_asc, name_desc);
}

//cargo test --color=always --package rusty_pitchfork --test streams test_list_search_by_dataset_id_params_are_not_ignored_by_domo -- --nocapture
#[test]
fn test_list_search_by_dataset_id_params_are_not_ignored_by_domo() {
    let domo = create_test_rusty_fork();
    let first_five_no_search = domo.list_streams(5i32, 0i32).unwrap();
    //    let search= domo.list_streams_by_owner(1704739518i32).unwrap();
    let search = domo
        .search_stream_by_dataset_id(&"d47f9e01-9032-4201-a8c6-e2facc714de3".to_owned())
        .unwrap();
    dbg!(first_five_no_search[0].id);
    dbg!(search[0].id);
    assert_ne!(first_five_no_search[0].id, search[0].id);
}

#[test]
fn test_list_fields_params_are_not_ignored_by_domo() {
    let domo = create_test_rusty_fork();
    let list_url = "v1/streams".to_owned();
    let mut no_fields_param = HashMap::new();
    no_fields_param.insert("limit".to_owned(), "5".to_owned());
    no_fields_param.insert("offset".to_owned(), "0".to_owned());
    let mut fields_dataset_params = HashMap::new();
    fields_dataset_params.insert("limit".to_owned(), "5".to_owned());
    fields_dataset_params.insert("offset".to_owned(), "0".to_owned());
    fields_dataset_params.insert("fields".to_owned(), "dataSet".to_owned());
    let mut fields_update_method_params = HashMap::new();
    fields_update_method_params.insert("limit".to_owned(), "5".to_owned());
    fields_update_method_params.insert("offset".to_owned(), "0".to_owned());
    fields_update_method_params.insert("fields".to_owned(), "updateMethod".to_owned());

    let mut fields_created_at_params = HashMap::new();
    fields_created_at_params.insert("limit".to_owned(), "5".to_owned());
    fields_created_at_params.insert("offset".to_owned(), "0".to_owned());
    fields_created_at_params.insert("fields".to_owned(), "createdAt".to_owned());
    let mut fields_modified_at_params = HashMap::new();
    fields_modified_at_params.insert("limit".to_owned(), "5".to_owned());
    fields_modified_at_params.insert("offset".to_owned(), "0".to_owned());
    fields_modified_at_params.insert("fields".to_owned(), "modifiedAt".to_owned());
    let no_fields_params = domo.domo_get(&list_url, &no_fields_param).unwrap();
    let dataset_fields = domo.domo_get(&list_url, &fields_dataset_params).unwrap();
    let update_method_fields = domo
        .domo_get(&list_url, &fields_update_method_params)
        .unwrap();
    let created_at_fields = domo.domo_get(&list_url, &fields_created_at_params).unwrap();
    let modified_at_fields = domo
        .domo_get(&list_url, &fields_modified_at_params)
        .unwrap();
    assert_ne!(dataset_fields, update_method_fields);
    assert_ne!(dataset_fields, created_at_fields);
    assert_ne!(dataset_fields, modified_at_fields);
    assert_ne!(update_method_fields, created_at_fields);
    assert_ne!(update_method_fields, modified_at_fields);
    assert_ne!(created_at_fields, modified_at_fields);
    assert_ne!(no_fields_params, dataset_fields);
    assert_ne!(no_fields_params, update_method_fields);
    assert_ne!(no_fields_params, created_at_fields);
    assert_ne!(no_fields_params, modified_at_fields);
}

#[test]
fn test_get_stream_details() {}

#[test]
fn test_create_stream() {}

#[test]
fn test_create_execution() {}

#[test]
fn test_commit_execution() {}

#[test]
fn test_abort_execution() {}

#[test]
fn test_get_executions_list() {}

#[test]
fn test_get_steams_list() {}

#[test]
fn test_delete_stream() {}

#[test]
fn test_update_stream_meta() {}

#[test]
fn test_upload_data_part_str() {}

#[test]
fn test_upload_data_part_file() {}

#[test]
fn test_stream_e2e() {
    let domo = create_test_rusty_fork();
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

    let stream = domo.create_stream(stream_ds).unwrap();
    let e = domo.create_stream_execution(stream.id).unwrap();
    domo.upload_data_part(stream.id, e.id, 1i32, &csv).unwrap();
    let _commit = domo.commit_execution(stream.id, e.id).unwrap();
    let _ = domo.delete_stream(stream.id);
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

fn create_test_rusty_fork() -> RustyPitchfork {
    dotenv::dotenv().ok();
    let domo_client_id = env::var("DOMO_CLIENT_ID").unwrap();
    let domo_secret = env::var("DOMO_SECRET").unwrap();
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .build();
    RustyPitchfork::default().auth_manager(client_creds).build()
}
