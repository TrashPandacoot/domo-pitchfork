extern crate csv;
extern crate docopt;
extern crate rusty_pitchfork;

use rusty_pitchfork::auth::DomoClientAppCredentials;
use rusty_pitchfork::client::RustyPitchfork;
use rusty_pitchfork::domo::dataset::*;
use rusty_pitchfork::domo::stream::*;

use std::collections::HashMap;
use std::env;
use std::fs::File;

use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &'static str = "
Interact with Domo Datasets via Stream API.

When uploading column order will be the same order as the input and will fail
if the order doesn't match the stream schema in Domo. The schema can automatically
be updated to match the upload source with the '--update-schema' flag.

Usage:
    ripdomo stream add <name> <file> [-i | --update-method-replace ] [-e | --empty]
    ripdomo stream remove [options]
    ripdomo stream up [options] <file>
    ripdomo stream info [options]
    ripdomo stream list [options]
    ripdomo stream search
    ripdomo stream execution <exid> ( -d <id> | --stream-id <id>) ( -a | --abort )
    ripdomo stream execution <exid> ( -d <id> | --stream-id <id>) ( -c | --commit )
    ripdomo stream execution ( -d <id> | --stream-id <id>) [ -l <limit> | --limit <limit>] [ -s <offset> | --skip <offset>]
    ripdomo stream --help
    ripdomo stream -h

stream options:
    -a, --abort                     Abort Stream Execution
    -c, --commit                    Commit Stream Execution
    -u, --update-schema             When uploading to Domo, check the schema matches the input. Update if needed.
    -i, --update-method-replace     Import via replace instead of appending.
    -e, --empty                     Create stream with schema based on the file, but don't upload file after creation.
    -p, --part <pnum>               Upload Part Number
    -d, --stream-id <id>            Domo stream <id> to upload to.
    -l, --limit <limit>             Limit to return in list streams.
    -s, --skip <offset>             Offset to start Domo Dataset List from.

common options:
    -h, --help              Display this message
";

#[derive(Deserialize)]
struct Args {
    cmd_add: bool,
    cmd_remove: bool,
    cmd_up: bool,
    cmd_info: bool,
    cmd_search: bool,
    cmd_list: bool,
    cmd_execution: bool,
    arg_name: Option<String>,
    arg_file: Option<String>,
    arg_exid: Option<i32>,
    flag_abort: bool,
    flag_commit: bool,
    flag_update_schema: bool,
    flag_update_method_replace: bool,
    flag_empty: bool,
    flag_limit: Option<i32>,
    flag_skip: Option<i32>,
    flag_part: Option<i32>,
    flag_stream_id: Option<i32>,
    // flag_output: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.cmd_info {
        args.stream_info()
    } else if args.cmd_search {
        args.stream_search()
    } else if args.cmd_list {
        args.stream_list()
    } else if args.cmd_add {
        args.stream_add()
    } else if args.cmd_remove {
        args.stream_remove()
    } else if args.cmd_up {
        args.stream_up()
    } else if args.cmd_execution {
        args.stream_execution()
    } else {
        unreachable!();
    }
}

impl Args {
    fn stream_search(&self) -> CliResult<()> {
        // let lim = match &self.flag_limit {
        //     Some(num) => *num,
        //     None => 500,
        // };

        // let skip = match &self.flag_skip {
        //     Some(num) => *num,
        //     None => 0,
        // };

        let domo = get_client();
        let streams = domo.search_stream_by_dataset_id(&"d47f9e01-9032-4201-a8c6-e2facc714de3")?; //list_streams_by_owner(1704739518i32)?;
        println!("{:#?}", streams);
        Ok(())
    }

    fn stream_list(&self) -> CliResult<()> {
        let lim = match &self.flag_limit {
            Some(num) => *num,
            None => 500,
        };

        let skip = match &self.flag_skip {
            Some(num) => *num,
            None => 0,
        };

        let domo = get_client();
        let streams = domo.list_streams(lim, skip)?;
        println!("{:#?}", streams);
        Ok(())
    }

    fn stream_info(&self) -> CliResult<()> {
        let stream_id = match &self.flag_stream_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Stream Id Given"),
        };

        let domo = get_client();
        let info = domo.stream_details(stream_id)?;
        println!("{:#?}", info);
        Ok(())
    }

    fn stream_add(&self) -> CliResult<()> {
        let up_method = match &self.flag_update_method_replace {
            false => "APPEND".to_string(),
            true => "REPLACE".to_string(),
        };
        let path = match &self.arg_file {
            Some(input) => input.to_string(),
            _ => return fail!("No CSV Input"),
        };

        let file = File::open(&path)?;
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

        // Use Vec to make sure columns are in the correct order for the schema.
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

        // println!("{:#?}", col_headers);

        let ds_name = match &self.arg_name {
            Some(name) => name.to_owned(),
            None => "Rip Domo Dataset".to_string(),
        };

        let ds_desc = "Rip Domo Generated Dataset".to_string();

        let ds = DatasetSchema {
            name: ds_name,
            description: ds_desc,
            rows: 0,
            schema: Schema {
                columns: col_headers,
            },
        };
        let stream_ds = StreamDatasetSchema {
            dataset_schema: ds,
            update_method: up_method,
        };

        let domo = get_client();
        let new_stream = domo.create_stream(stream_ds)?;
        println!("Stream ID: {}", &new_stream.id);

        if !(&self.flag_empty) {
            let stream_ex = domo.create_stream_execution(new_stream.id)?;
            domo.upload_data_part_file(new_stream.id, stream_ex.id, 1, &path)
                .expect("Upload Data Part Failed");
            // TODO: Handle case where data is uploaded by commit fails to execute better
            let _commit = domo.commit_execution(new_stream.id, stream_ex.id)?;
            println!("Data Imported to Domo Successfully");
        }

        Ok(())
    }

    /// Delete a Domo Stream. Note: this doesn't delete the Dataset for the Stream.
    fn stream_remove(&self) -> CliResult<()> {
        println!("DS remove");
        let ds_id = match &self.flag_stream_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Dataset Id Given"),
        };

        let domo = get_client();
        domo.delete_stream(ds_id)?;
        Ok(())
    }

    /// Upload to Domo Stream. This method can result in loss of data if update schema flag is set
    /// and the stream Update Method is set to append.
    fn stream_up(&self) -> CliResult<()> {
        let stream_id = match &self.flag_stream_id {
            Some(id) => id.to_owned(),
            None => return fail!("No Dataset ID specified"),
        };
        let part_num = match self.flag_part {
            Some(num) => num,
            None => return fail!("No Data Part Number specified"),
        };

        let path = match &self.arg_file {
            Some(input) => input.to_owned(),
            _ => return fail!("No CSV Input"),
        };

        let update_schema = match &self.flag_update_schema {
            true => true,
            false => false,
        };

        let file = File::open(&path)?;
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

        // Use Vec to make sure columns are in the correct order for the schema.
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

        let domo = get_client();
        if update_schema {
            let domo_schema = domo.stream_details(stream_id).unwrap().dataset;
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
            let ds = DatasetSchema {
                name: ds_name,
                description: ds_desc,
                rows: 0,
                schema: Schema {
                    columns: col_headers,
                },
            };

            let ds_id = domo_schema.id;
            // match domo_schema.schema {
            //     Some(s) => {
            //         if stream_schema_diff(&ds.schema, &s) {
            // TODO: fix update stream dataset method to update underlying stream ds
            domo.update_dataset_meta(&ds_id, ds)?;
            //             eprintln!("Updating Schema not yet implemented for streams")
            //         }
            //     }
            //     None => return fail!("Error Getting Schema from Domo"),
            // }
        }

        let execution = domo.create_stream_execution(stream_id)?;

        domo.upload_data_part_file(stream_id, execution.id, part_num, &path)
            .expect("Upload Data Part From File Failed");
        // TODO: this is awkward if you're passing more than one data part in. Do I make a separate commit command or a less manual partitioning? leaning towards auto partitioning with loop
        let _commit = domo
            .commit_execution(stream_id, execution.id)
            .expect("Commiting Stream Execution Failed");

        println!("Dataset Imported into Domo");
        Ok(())
    }
    fn stream_execution(&self) -> CliResult<()> {
        let stream_id = match &self.flag_stream_id {
            Some(id) => *id,
            None => return fail!("No Stream Id Provided"),
        };
        let domo = get_client();

        if self.flag_abort {
            let exid = match &self.arg_exid {
                Some(id) => *id,
                None => return fail!("No Stream Execution Id Provided"),
            };
            domo.abort_stream_execution(stream_id, exid)?;
            println!("Stream {} Execution {} Aborted", stream_id, exid);
            Ok(())
        } else if self.flag_commit {
            let exid = match &self.arg_exid {
                Some(id) => *id,
                None => return fail!("No Stream Execution Id Provided"),
            };
            let c = domo.commit_execution(stream_id, exid)?;
            println!("Commited Execution: \n {:?}", c);
            Ok(())
        } else {
            let lim = match &self.flag_limit {
                Some(num) => *num,
                None => 50,
            };
            let offset = match &self.flag_skip {
                Some(num) => *num,
                None => 0,
            };
            let executions = domo.list_stream_executions(i64::from(stream_id), lim, offset)?;
            println!("{:?}", executions);
            Ok(())
        }
    }
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
