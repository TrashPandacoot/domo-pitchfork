use crate::CliCommand;
use crate::CliResult;
use crate::CliError;
use csv;
use log::{debug, info, trace};
use domo_pitchfork::auth::DomoClientAppCredentials;
use domo_pitchfork::auth::DomoScope;
use domo_pitchfork::pitchfork::DomoPitchfork;
use domo_pitchfork::domo::dataset::check_field_type;
use domo_pitchfork::domo::dataset::Column;
use domo_pitchfork::domo::dataset::DatasetSchema;
use domo_pitchfork::domo::dataset::FieldType;
use domo_pitchfork::domo::dataset::Record;
use domo_pitchfork::domo::dataset::Schema;
use domo_pitchfork::domo::stream::{StreamSearchQuery, StreamDatasetSchema};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]

pub(crate) enum StreamCmd {
    #[structopt(name = "create")]
    Add(StreamAdd),
    #[structopt(name = "list")]
    List(StreamList),
    #[structopt(name = "search")]
    Search(StreamSearch),
    /// Delete a Domo Stream. Note: this doesn't delete the Dataset for the Stream.
    #[structopt(name = "delete")]
    Remove(StreamRemove),
    #[structopt(name = "execution")]
    Execution(StreamExecution),
    #[structopt(name = "modify")]
    Modify(StreamModify),
    #[structopt(name = "info")]
    Info(StreamInfo),
}
impl CliCommand for StreamCmd {
    fn run(self) -> CliResult<()> {
        match self {
            StreamCmd::Add(a) => a.run(),
            StreamCmd::List(a) => a.run(),
            StreamCmd::Modify(a) => a.run(),
            StreamCmd::Remove(a) => a.run(),
            StreamCmd::Execution(a) => a.run(),
            StreamCmd::Search(a) => a.run(),
            StreamCmd::Info(a) => a.run(),
        }
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct StreamAdd {
    pub name: String,
    #[structopt(short = "d", long = "description")]
    pub description: Option<String>,
    #[structopt(short = "a", long = "appended")]
    pub is_append_update_method: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    pub file: PathBuf,
}
impl CliCommand for StreamAdd {
    fn run(self) -> CliResult<()> {
        let up_method = match &self.is_append_update_method {
            false => "APPEND".to_string(),
            true => "REPLACE".to_string(),
        };

        let file = File::open(&self.file)?;
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

        trace!("Inferring schema from file");
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

        debug!("{:#?}", col_headers);

        let col_count = col_headers.len();
        let ds = DatasetSchema {
            name: self.name,
            description: self
                .description
                .unwrap_or_else(|| "ripdomo generated stream api dataset".to_owned()),
            rows: 0,
            schema: Schema {
                columns: col_headers,
            },
        };
        let stream_ds = StreamDatasetSchema {
            dataset_schema: ds,
            update_method: up_method,
        };

        let token = token();
        let domo = DomoPitchfork::with_token(&token);
        let new_stream = domo.streams().create(&stream_ds)?;
        info!("Created stream for dataset with {} columns", col_count);
        println!("Stream ID: {}", &new_stream.id);

        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct StreamList {
    #[structopt(short = "l", long = "limit", default_value = "50")]
    //TODO: check what domo doc says is range
    limit: u32,
    #[structopt(short = "s", long = "offset", default_value = "0")]
    offset: u32,
}
impl CliCommand for StreamList {
    fn run(self) -> CliResult<()> {
        let token = token();
        let domo = DomoPitchfork::with_token(&token);
        let streams = domo.streams().list(self.limit, self.offset)?;
        info!("Retreived {} streams", streams.len());
        println!("{:#?}", streams);
        Ok(())
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct StreamSearch {
    #[structopt(short = "id", long = "datasetid")]
    dataset_id: Option<String>,
    #[structopt(short = "owner", long = "ownerid")]
    dataset_owner_id: Option<u64>,
}
impl CliCommand for StreamSearch {
    fn run(self) -> CliResult<()> {
        let token = token();
        let domo = DomoPitchfork::with_token(&token);
        let query = if self.dataset_id.is_some() {
            StreamSearchQuery::DatasetId(self.dataset_id.as_ref().unwrap().clone())
        } else if self.dataset_owner_id.is_some() {
            StreamSearchQuery::DatasetOwnerId(*self.dataset_owner_id.as_ref().unwrap())
        } else {
            return Err(CliError::from("No stream search parameters provided"))
        };
        let streams = domo.streams().search(query)?; //list_streams_by_owner(1704739518i32)?;
        println!("{:#?}", streams);
        Ok(())
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct StreamInfo {
    #[structopt(name = "stream id")]
    stream_id: u64,
}
impl CliCommand for StreamInfo {
    fn run(self) -> CliResult<()> {
        let token = token();
        let domo = DomoPitchfork::with_token(&token);
        let info = domo.streams().info(self.stream_id)?;
        println!("{:#?}", info);
        Ok(())
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct StreamRemove {
    #[structopt(name = "stream id")]
    stream_id: u64,
}
impl CliCommand for StreamRemove {
    fn run(self) -> CliResult<()> {
        println!("DS remove");

        let token = token();
        let domo = DomoPitchfork::with_token(&token);
        domo.streams().delete(self.stream_id)?;
        Ok(())
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct StreamModify {}
impl CliCommand for StreamModify {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}

#[derive(StructOpt, Debug)]

pub(crate) enum StreamExecution {
    #[structopt(name = "create")]
    Create(StreamExecutionCreate),
    #[structopt(name = "commit")]
    Commit(StreamExecutionCommit),
    #[structopt(name = "abandon")]
    Abandon(StreamExecutionAbandon),
    #[structopt(name = "info")]
    Info(StreamExecutionInfo),
    #[structopt(name = "list")]
    List(StreamExecutionList),
}
impl CliCommand for StreamExecution {
    fn run(self) -> CliResult<()> {
        match self {
            StreamExecution::Create(a) => a.run(),
            StreamExecution::Commit(a) => a.run(),
            StreamExecution::Abandon(a) => a.run(),
            StreamExecution::List(a) => a.run(),
            StreamExecution::Info(a) => a.run(),
        }
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct StreamExecutionCreate {}
impl CliCommand for StreamExecutionCreate {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct StreamExecutionCommit {}
impl CliCommand for StreamExecutionCommit {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct StreamExecutionAbandon {}
impl CliCommand for StreamExecutionAbandon {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct StreamExecutionInfo {}
impl CliCommand for StreamExecutionInfo {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct StreamExecutionList {}
impl CliCommand for StreamExecutionList {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}

/// returns a token to use with the `DomoPitchfork` client to use to interact with the Domo API.
fn token() -> String {
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
            buzz: false,
            account: false,
            workflow: false,
        })
        .build();
    client_creds.get_access_token()
}
