use domo_pitchfork_derive::Domo;
use domo_pitchfork::domo::dataset::{Column, Schema, DomoSchema};

#[derive(Domo)]
pub struct Example {
    pub name: String,
    pub opt_name: Option<String>,
    #[domo(name = "Number")]
    pub number: usize,
    #[domo(name = "Float", column_type = "DOUBLE")]
    pub float: f32,
    #[domo(column_type = "DOUBLE")]
    pub opt_float: Option<f32>,
    pub opt_number: Option<i32>,
    #[domo("DATETIME")]
    pub field_to_cast_as_date: String,
}

#[test]
fn example_ds_schema() {
    let schema: Schema = Example::domo_dataset_schema();
    let expected_schema = Schema {
        columns: vec![
            Column {
                name: "name".to_string(),
                column_type: "STRING".to_string(),
            },
            Column {
                name: "opt_name".to_string(),
                column_type: "STRING".to_string(),
            },
            Column {
                name: "Number".to_string(),
                column_type: "LONG".to_string(),
            },
            Column {
                name: "Float".to_string(),
                column_type: "DOUBLE".to_string(),
            },
            Column {
                name: "opt_float".to_string(),
                column_type: "DOUBLE".to_string(),
            },
            Column {
                name: "opt_number".to_string(),
                column_type: "LONG".to_string(),
            },
            Column {
                name: "field_to_cast_as_date".to_string(),
                column_type: "DATETIME".to_string(),
            },
        ]
    };
    assert_eq!(schema, expected_schema);
}