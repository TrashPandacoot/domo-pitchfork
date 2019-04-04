use rusty_pitchfork::auth::DomoScope;
use crate::CliCommand;
use log::{info, trace};
use rusty_pitchfork::auth::DomoClientAppCredentials;
use rusty_pitchfork::client::RustyPitchfork;
use rusty_pitchfork::domo::dataset::*;
use std::path::PathBuf;
use structopt::StructOpt;

use std::collections::HashMap;
use std::env;
use std::fs::File;

use crate::CliResult;
#[derive(StructOpt, Debug)]
pub(crate) enum DatasetCmd {
    /// Add a new Dataset to Domo.
    #[structopt(name = "create")]
    Add(DatasetAdd),
    /// List Domo Datasets
    #[structopt(name = "list")]
    List(DatasetList),
    /// Edit properties for a given dataset.
    #[structopt(name = "modify")]
    Modify(DatasetModify),
    /// Remove a Dataset from Domo. CAUTION: This is irreversable!
    #[structopt(name = "delete")]
    Remove(DatasetRemove),
    /// Update data for a given dataset.
    #[structopt(name = "update")]
    Update(DatasetUpdate),
    /// Get data for a given dataset as a CSV. i.e. Export Data
    #[structopt(name = "data")]
    Data(DatasetData),
    /// Get info about a dataset.
    #[structopt(name = "info")]
    Info(DatasetInfo),
}
impl CliCommand for DatasetCmd {
    fn run(self) -> CliResult<()> {
        match self {
            DatasetCmd::Add(a) => a.run(),
            DatasetCmd::List(a) => a.run(),
            DatasetCmd::Modify(a) => a.run(),
            DatasetCmd::Remove(a) => a.run(),
            DatasetCmd::Update(a) => a.run(),
            DatasetCmd::Data(a) => a.run(),
            DatasetCmd::Info(a) => a.run(),
        }
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct DatasetModify {
    // #[structopt(name = "dataset id")]
    /// Dataset ID for the target for modification.
    dataset_id: String,
    /// New name for the Dataset.
    #[structopt(long = "name")]
    pub name: Option<String>,
    /// New Optional Description for the dataset.
    #[structopt(short = "d", long = "description")]
    pub description: Option<String>,
    // /// Set the update method for this dataset to be append instead of replace.
    // #[structopt(short = "a", long = "appended")]
    // pub is_append_update_method: bool,
}

impl CliCommand for DatasetModify {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct DatasetList {
    #[structopt(short = "l", long = "limit", default_value = "50")]
    //TODO: check what domo doc says is range
    limit: i32,
    #[structopt(short = "s", long = "offset", default_value = "0")]
    offset: i32,
}
impl CliCommand for DatasetList {
    /// List datasets given a limit and number to skip. First 50 if no parameters are given.
    fn run(self) -> CliResult<()> {
        let domo = get_client();;
        let datasets = domo.list_datasets(self.limit, self.offset)?;
        info!("Retrieved {} datasets", datasets.len());
        println!("{:?}", datasets);
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct DatasetInfo {
    #[structopt(name = "dataset id")]
    dataset_id: String,
}
impl CliCommand for DatasetInfo {
    /// Print Info for a given dataset.
    fn run(self) -> CliResult<()> {
        let domo = get_client();
        let info = domo.dataset(&self.dataset_id)?;
        println!("{:?}", info);
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct DatasetData {
    #[structopt(name = "dataset id")]
    dataset_id: String,
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    file: Option<PathBuf>,
}
impl CliCommand for DatasetData {
    /// Export a given data set to a file at the given path.
    fn run(self) -> CliResult<()> {
        let domo = get_client();
        let data = domo.export_data(&self.dataset_id)?;

        if self.file.is_some() {
            //TODO: Write output to file!
            unimplemented!();
        } else {
            println!("{}", data);
        }
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct DatasetAdd {
    /// Name for the Dataset being created.
    pub name: String,
    /// Optional Description for the dataset being created.
    #[structopt(short = "d", long = "description")]
    pub description: Option<String>,
    /// Set the update method for this dataset to be append instead of replace.
    #[structopt(short = "a", long = "appended")]
    pub is_append_update_method: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    pub file: PathBuf,
}
impl CliCommand for DatasetAdd {
    /// Create a new Domo dataset.
    fn run(self) -> CliResult<()> {
        let file = File::open(self.file)?;
        let mut rdr = csv::Reader::from_reader(file);
        let headers = rdr.headers()?.clone();

        // Create HashMap to create Column, FieldType (k, v)
        let mut columns = HashMap::new();
        let mut col_headers = Vec::new();

        // Go through Csv row by row and create col, field type map
        for result in rdr.deserialize() {
            let record: Record = result?;
            if let Err(err) = check_field_type(&record, &mut columns) {
                println!("Error parsing column types. {}", err);
            }
        }

        // Loop through Vec of headers to ensure column order is correct
        for header in &headers {
            let typ = columns.get(&header.to_string());

            let typ_str = match typ {
                Some(FieldType::TUnicode) => "STRING".to_string(),
                Some(FieldType::TFloat) => "DOUBLE".to_string(),
                Some(FieldType::TInteger) => "LONG".to_string(),
                _ => "STRING".to_string(),
            };
            col_headers.push(Column {
                column_type: typ_str,
                name: header.to_string(),
            });
        }

        let dataset = DatasetSchema {
            name: self.name,
            description: self
                .description
                .unwrap_or_else(|| "ripdomo generated dataset".to_owned()),
            rows: 0,
            schema: Schema {
                columns: col_headers,
            },
        };

        let domo = get_client();
        let new_ds = domo.create_dataset(dataset)?;
        println!("Dataset ID: {}", new_ds.id);

        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct DatasetRemove {
    #[structopt(name = "dataset id")]
    dataset_id: String,
}
impl CliCommand for DatasetRemove {
    /// Delete a given Domo dataset.
    fn run(self) -> CliResult<()> {
        println!("DS remove");
        let domo = get_client();
        domo.delete_dataset(&self.dataset_id)?;
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct DatasetUpdate {
    #[structopt(name = "dataset id")]
    dataset_id: String,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: PathBuf,
    /// Compare upload data's schema with dataset's schema and update dataset schema if necessary.
    #[structopt(long = "update-schema")]
    pub should_update_schema_if_changed: bool,
    // #[structopt(short = "f", long = "file", parse(from_os_str))]
    // csvdata: Option<String>,
}
impl CliCommand for DatasetUpdate {
    /// Upload data to a Domo dataset.
    fn run(self) -> CliResult<()> {
        let file = File::open(&self.file)?;
        let mut rdr = csv::Reader::from_reader(file);

        // Create HashMap to create Column, FieldType (k, v)
        let mut columns = HashMap::new();

        // Go through Csv row by row and create col, field type map
        for result in rdr.deserialize() {
            let record: Record = result?;
            if let Err(err) = check_field_type(&record, &mut columns) {
                println!("Error parsing column types. {}", err);
            }
        }

        let domo = get_client();
        // FIXME: need to update this to ensure schema update has columns in correct order since HashMap isn't ordered.
        if self.should_update_schema_if_changed {
            let domo_schema = domo.dataset(&self.dataset_id).unwrap();
            let ds_name = domo_schema
                .name
                .expect("No Dataset Name on Domo Dataset Retrieved");

            let ds_desc = domo_schema
                .description
                .expect("No Dataset Name on Domo Dataset Retrieved");

            let dataset = DatasetSchema::from_hashmap(ds_name, ds_desc, &columns);
            match domo_schema.schema {
                Some(s) => {
                    if dataset_schema(&dataset.schema, &s) {
                        domo.update_dataset_meta(&self.dataset_id, dataset)?;
                    }
                }
                None => return fail!("Error Getting Schema from Domo"),
            }
        }

        domo.replace_data_with_file(&self.dataset_id, &self.file.to_string_lossy())?;

        println!("Dataset Imported into Domo");
        Ok(())
    }
}

/// Compare schemas and check if they're the same.
/// TODO: Remove this since it's not necessary to make a separate method for
fn dataset_schema(ds: &Schema, domo_schema: &Schema) -> bool {
    ds == domo_schema
}

/// returns a `RustyPitchfork` client to use to interact with the Domo API.
fn get_client() -> RustyPitchfork {
    let domo_client_id = env::var("DOMO_CLIENT_ID").unwrap();
    let domo_secret = env::var("DOMO_SECRET").unwrap();
    trace!("Authenticating with Domo as Client ID: {}", domo_client_id);
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .client_scope(DomoScope{
            data: true,
            user: false,
            audit: false,
            dashboard: false,
        })
        .build();
    RustyPitchfork::default().auth_manager(client_creds).build()
}
