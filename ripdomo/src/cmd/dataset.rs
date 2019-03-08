extern crate csv;
extern crate docopt;
extern crate rusty_pitchfork;

use rusty_pitchfork::auth::DomoClientAppCredentials;
use rusty_pitchfork::domo::dataset::*;
use rusty_pitchfork::client::RustyPitchfork;

use std::collections::HashMap;
use std::env;
use std::fs::File;

use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &'static str = "
Interact with Domo Dataset API.

When uploading column order will be the same order as the input and will fail
if the order doesn't match the dataset schema in Domo. The schema can automatically
be updated to match the upload source with the '--update-schema' flag.

Usage:
    ripdomo dataset add <name> <input>
    ripdomo dataset remove [options]
    ripdomo dataset up [options] <input>
    ripdomo dataset export [options]
    ripdomo dataset info [options]
    ripdomo dataset list [options]
    ripdomo dataset --help
    ripdomo dataset -h

dataset options:
    -u, --update-schema     When uploading to Domo, check the schema matches the input.
                            If it doesn't, update Domo schema to match input.
    -d, --dataset-id <id>   Domo dataset <id> to upload to.
    -l, --limit <limit>     Limit to return in list datasets.
    -s, --skip <offset>     Offset to start Domo Dataset List from.

common options:
    -h, --help              Display this message
    -o, --output <file>     Write output to <file> instead of stdout
";

#[derive(Deserialize)]
struct Args {
    cmd_add: bool,
    cmd_remove: bool,
    cmd_up: bool,
    cmd_export: bool,
    cmd_info: bool,
    cmd_list: bool,
    arg_name: Option<String>,
    arg_input: Option<String>,
    flag_limit: Option<i32>,
    flag_skip: Option<i32>,
    flag_update_schema: bool,
    flag_dataset_id: Option<String>,
    flag_output: Option<String>,
}

/// All Dataset related commands.
#[derive(Debug, Deserialize)]
enum DatasetCommand {
    /// List Datasets.
    List,
    /// Get details for a given dataset.
    Info,
    /// Export data from existing Domo dataset.
    Export,
    /// Create a new Domo dataset.
    Add,
    /// Upload data to a Domo dataset.
    Upload,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.cmd_info {
        args.dataset_info()
    } else if args.cmd_list {
        args.dataset_list()
    } else if args.cmd_add {
        args.dataset_add()
    } else if args.cmd_export {
        args.dataset_export()
    } else if args.cmd_remove {
        args.dataset_remove()
    } else if args.cmd_up {
        args.dataset_up()
    } else {
        unreachable!();
    }
}

impl Args {
    /// List datasets given a limit and number to skip. First 500 if no parameters are given.
    fn dataset_list(&self) -> CliResult<()> {
        let lim = match &self.flag_limit {
            Some(num) => *num,
            None => 500,
        };

        let skip = match &self.flag_skip {
            Some(num) => *num,
            None => 0,
        };

        let domo = get_client();
        let datasets = domo.list_datasets(lim, skip)?;
        println!("{:?}", datasets);

        Ok(())
    }

    /// Print Info for a given dataset.
    fn dataset_info(&self) -> CliResult<()> {
        let ds_id = match &self.flag_dataset_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Dataset Id Given"),
        };

        let domo = get_client();
        let info = domo.dataset(&ds_id)?;
        println!("{:?}", info);
        Ok(())
    }

    /// Export a given data set to a file at the given path.
    fn dataset_export(&self) -> CliResult<()> {
        let ds_id = match &self.flag_dataset_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Dataset Id Given"),
        };

        let domo = get_client();
        let data = domo.export_data(&ds_id)?;

        if let Some(path) = &self.flag_output {
            println!("TODO: io write to file path: {}", path);
        } else {
            println!("{}", data);
        }
        println!("DS export");
        Ok(())
    }

    /// Create a new Domo dataset.
    fn dataset_add(&self) -> CliResult<()> {
        let path = match &self.arg_input {
            Some(input) => input.to_owned(),
            _ => return fail!("No CSV Input"),
        };
        println!("{}", path);
        let file = File::open(path)?;
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

        let ds_name = match &self.arg_name {
            Some(name) => name.to_owned(),
            None => "Rip Domo Dataset".to_string(),
        };

        let ds_desc = "Rip Domo Generated Dataset".to_string();

        let dataset = DatasetSchema {
            name: ds_name,
            description: ds_desc,
            rows: 0,
            schema: Schema {
                columns: col_headers,
            },
        };

        //println!("{:?}", dataset);

        let domo = get_client();
        let new_ds = domo.create_dataset(dataset)?;
        println!("Dataset ID: {}", new_ds.id);

        Ok(())
    }

    /// Delete a given Domo dataset.
    fn dataset_remove(&self) -> CliResult<()> {
        println!("DS remove");
        let ds_id = match &self.flag_dataset_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Dataset Id Given"),
        };

        let domo = get_client();
        domo.delete_dataset(&ds_id)?;
        Ok(())
    }

    /// Upload data to a Domo dataset.
    fn dataset_up(&self) -> CliResult<()> {
        let dataset_id = match &self.flag_dataset_id {
            Some(id) => id.to_string(),
            None => return fail!("No Dataset ID specified"),
        };

        let path = match &self.arg_input {
            Some(input) => input.to_owned(),
            _ => return fail!("No CSV Input"),
        };

        let update_schema = match &self.flag_update_schema {
            true => true,
            false => false,
        };

        let file = File::open(&path)?;
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
        if update_schema {
            let domo_schema = domo.dataset(&dataset_id).unwrap();
            let ds_name = match &self.arg_name {
                Some(name) => name.to_owned(),
                None => domo_schema
                    .name
                    .expect("No Dataset Name on Domo Dataset Retrieved"),
            };

            let ds_desc = match &self.arg_name {
                Some(name) => name.to_owned(),
                None => domo_schema
                    .description
                    .expect("No Dataset Name on Domo Dataset Retrieved"),
            };

            let dataset = DatasetSchema::from_hashmap(ds_name, ds_desc, &columns);
            match domo_schema.schema {
                Some(s) => {
                    if dataset_schema(&dataset.schema, &s) {
                        domo.update_dataset_meta(&dataset_id, dataset)?;
                    }
                }
                None => return fail!("Error Getting Schema from Domo"),
            }
        }

        domo.replace_data_with_file(&dataset_id, &path)?;

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
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .build();
    RustyPitchfork::default().auth_manager(client_creds).build()
}
