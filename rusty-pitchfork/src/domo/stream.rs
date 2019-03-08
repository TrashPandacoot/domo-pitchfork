use crate::domo::dataset::Dataset;
use crate::domo::dataset::DatasetSchema;
use serde::{Deserialize, Serialize};

// [Stream Object](https://developer.domo.com/docs/streams-api-reference/streams
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamDataset {
    pub id: i32,
    #[serde(rename = "dataSet")]
    pub dataset: Dataset,
    #[serde(rename = "updateMethod")]
    pub update_method: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "lastExecution")]
    pub last_execution: Option<StreamExecution>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamExecution {
    pub id: i32,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "endedAt")]
    pub ended_at: Option<String>,
    #[serde(rename = "currentState")]
    pub current_state: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamDatasetSchema {
    #[serde(rename = "dataSet")]
    pub dataset_schema: DatasetSchema,
    #[serde(rename = "updateMethod")]
    pub update_method: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewStreamDataset {
    #[serde(rename = "dataSet")]
    pub dataset: StreamDatasetSchema,
}
//public enum UpdateMethod
//{
//    APPEND,
//    REPLACE
//}

//TODO: impl UpdateMethod

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json;
//     #[test]
//     fn test_dataset_schema_serialization() {
//         let c = Column {
//             column_type: "STRING".to_string(),
//             name: "column name".to_string(),
//         };
//         let s = Schema { columns: vec![c] };
//         let d_schema = DatasetSchema {
//             name: "test dataset".to_string(),
//             description: "test description".to_string(),
//             rows: 0,
//             schema: s,
//         };
//         let expected = json!({
// 								"name": "test dataset",
// 								"description": "test description",
// 								"rows": 0,
// 								"schema": {
// 									"columns": [{
// 										"type": "STRING",
// 										"name": "column name"
// 									}]
// 								},
// 							});

//         let v = serde_json::to_value(d_schema).unwrap();
//         assert_eq!(v, expected);
//     }

//     #[test]
//     fn test_fieldtype_merge() {
//         assert!(false);
//     }

//     #[test]
//     fn test_fieldtype_from_sample() {
//         assert!(false);
//     }

//     #[test]
//     fn test_check_fieldtype() {
//         assert!(false);
//     }

//     #[test]
//     fn test_schema_from_hashmap() {
//         assert!(false);
//     }

//     #[test]
//     fn test_datasetschema_from_hashmap() {
//         assert!(false);
//     }

// }
