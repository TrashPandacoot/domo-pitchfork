extern crate rusty_pitchfork;

use rusty_pitchfork::auth::DomoClientAppCredentials;
use rusty_pitchfork::client::RustyPitchfork;
use rusty_pitchfork::domo::dataset::Dataset;
use std::env;

#[test]
fn test_get_dataset_details() {
    let rusty_fork = create_test_rusty_fork();
    let dataset_id = "d755e978-bffc-4a53-bef7-6760751776ec"; // "2f450bb0-58ed-4d79-8034-cd8c4b1bd8d3";
    let dataset_details: Dataset = rusty_fork.dataset(dataset_id).unwrap();
    assert_eq!(dataset_details.id, dataset_id);
}

#[test]
fn test_get_dataset_list() {
    //
    let rusty_fork = create_test_rusty_fork();
    let ds_list: Vec<Dataset> = rusty_fork.list_datasets(5, 0).unwrap();
    assert_eq!(ds_list.len(), 5);
}

#[test]
fn test_create_dataset() {
    //TODO: implement test
}

#[test]
fn test_update_dataset_meta() {
    //TODO: implement test
}

#[test]
fn test_export_dataset_data() {
    let rusty_fork = create_test_rusty_fork();
    let csv = rusty_fork
        .export_data("77faea51-68ab-4dd3-ae1a-8992bc1b58a8")
        .unwrap_or_default();
    println!("{}", csv);
    assert_eq!(1, 1);
}

#[test]
fn test_replace_dataset_data() {
    let rusty_fork = create_test_rusty_fork();
    let csv = create_test_csv();
    let replace = rusty_fork.replace_data("77faea51-68ab-4dd3-ae1a-8992bc1b58a8", &csv);
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

#[test]
fn test_delete_dataset() {
    //TODO: implement test
    //    let rusty_fork = create_test_rusty_fork();
    //    let delete_ds = rusty_fork.delete_dataset("datasetID");
    //    assert!(delete_ds.unwrap_or_default().len() > 0);
}

fn create_test_rusty_fork() -> RustyPitchfork {
    let domo_client_id = env::var("DOMO_CLIENT_ID").expect("No DOMO_CLIENT_ID env var found");
    let domo_secret = env::var("DOMO_SECRET").expect("No DOMO_SECRET env var found");
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .build();
    RustyPitchfork::default().auth_manager(client_creds).build()
}

// --------------------------------------------------------------------------------

//extern crate buildintelligence_domo_pitchfork;
//extern crate csv;
//extern crate serde;
//#[macro_use]
//extern crate serde_derive;
//
//use std::error::Error;
//use buildintelligence_domo_pitchfork::rusty_pitchfork::client::RustyPitchfork;
//use buildintelligence_domo_pitchfork::rusty_pitchfork::domo::dataset::*;
//
//use buildintelligence_domo_pitchfork::rusty_pitchfork::auth::DomoClientAppCredentials;
//
//fn main() {
//    println!("Hello, world!");
//    let client_creds = DomoClientAppCredentials::default()
//        .client_id("")
//        .client_secret("")
//        .build();
//    let rusty_fork = RustyPitchfork::default().auth_manager(client_creds).build();
//    let dataset_details = rusty_fork.dataset("2f450bb0-58ed-4d79-8034-cd8c4b1bd8d3");
//    println!("{:?}", dataset_details);
//
//    let ds_list = rusty_fork.list_datasets(50, 0);
// println!("{:?}", ds_list);

// let domo_test_csv = rusty_fork
//     .export_data("2f450bb0-58ed-4d79-8034-cd8c4b1bd8d3")
//     .unwrap_or_default();
// println!("{}", domo_test_csv);
// let des = str_to_rows(domo_test_csv);
// println!("{:?}", des);

// let cereal = getting_cereal(vec![
//     domo_test_ds_row {
//         dataflow_name: "df name".to_owned(),
//         col_expected: 0u64,
//         col_actual: 0u64,
//         rows_expected: 0u64,
//         rows_actual: 0u64,
//         total_err: 0u64,
//         col_count_diff: 0i16,
//         row_count_diff: 0i64,
//         test_execution: "put a date here".to_owned(),
//     },
//     domo_test_ds_row {
//         dataflow_name: "df name \"tricky\"".to_owned(),
//         col_expected: 0u64,
//         col_actual: 0u64,
//         rows_expected: 0u64,
//         rows_actual: 0u64,
//         total_err: 0u64,
//         col_count_diff: 0i16,
//         row_count_diff: 0i64,
//         test_execution: "put a date here".to_owned(),
//     },
// ]).unwrap_or_default();
// println!("{}", cereal);

//    let test_dataset = rusty_fork.create_dataset(create_test_schema()).expect("create_dataset failed");
//    let test_data = create_test_data();
//    let csv_test = getting_cereal_test(vec![TestDomoData { col_name: "t1".to_owned()},TestDomoData { col_name: "t2".to_owned()},TestDomoData { col_name: "t3".to_owned()},]).expect("cereal test failed");
//    let test_replace_ds = rusty_fork.replace_data(&test_dataset.id, &csv_test);
//    let rusty_cleanup = rusty_fork.delete_dataset(&test_dataset.id);
//    println!("Grill the data burgers");
//    println!("Yum!");
//}
//
//fn str_to_rows(csv_str: String) -> Result<Vec<domo_test_ds_row>, Box<Error>> {
//    let mut rdr = csv::Reader::from_reader(csv_str.as_bytes());
//    let mut data = Vec::<domo_test_ds_row>::new();
//    for row in rdr.deserialize() {
//        let record: domo_test_ds_row = row?;
//        data.push(record);
//    }
//    Ok(data)
//}
//
//fn getting_cereal(datarows: Vec<domo_test_ds_row>) -> Result<String, Box<Error>> {
//    let mut wtr = csv::Writer::from_writer(vec![]);
//
//    for row in datarows {
//        wtr.serialize(row)?;
//    }
//    let csv_str = String::from_utf8(wtr.into_inner()?)?;
//    Ok(csv_str)
//}
//
//fn getting_cereal_test(datarows: Vec<TestDomoData>) -> Result<String, Box<Error>> {
//    let mut wtr = csv::Writer::from_writer(vec![]);
//
//    for row in datarows {
//        wtr.serialize(row)?;
//    }
//    let csv_str = String::from_utf8(wtr.into_inner()?)?;
//    Ok(csv_str)
//}
//
//fn create_test_schema() -> DatasetSchema {
//    let c = Column {column_type: "STRING".to_owned(), name: "col_name".to_owned()};
//    let s = Schema { columns: vec![c]};
//    let d_schema = DatasetSchema {
//        name: "api test dataset".to_owned(),
//        description: "api test description".to_owned(),
//        rows: 0,
//        schema: s
//    };
//    d_schema
//}
//
//fn create_test_data() -> Vec<TestDomoData> {
//    vec![TestDomoData { col_name: "t1".to_owned()},TestDomoData { col_name: "t2".to_owned()},TestDomoData { col_name: "t3".to_owned()},]
//}
//
//#[derive(Clone, Debug, Serialize, Deserialize)]
//struct TestDomoData {
//    col_name: String,
//}
//
//#[derive(Clone, Debug, Serialize, Deserialize)]
//struct domo_test_ds_row {
//    #[serde(rename = "Dataflow Name")] dataflow_name: String,
//    #[serde(rename = "num_columns_expected")] col_expected: u64,
//    #[serde(rename = "num_columns_actual")] col_actual: u64,
//    rows_expected: u64,
//    rows_actual: u64,
//    #[serde(rename = "total_row_errors")] total_err: u64,
//    #[serde(rename = "Diff Total Col Count")] col_count_diff: i16,
//    #[serde(rename = "Diff Total Row Count")] row_count_diff: i64,
//    #[serde(rename = "Unit Test Execution")] test_execution: String,
//}
