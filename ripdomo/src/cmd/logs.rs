use rusty_pitchfork::auth::{DomoScope, DomoClientAppCredentials};
use rusty_pitchfork::client::RustyPitchfork;
use crate::CliCommand;
use crate::CliResult;
use structopt::StructOpt;
use std::env;
#[derive(StructOpt, Debug)]
pub(crate) enum LogsCmd {
    List(LogsList)
}
impl CliCommand for LogsCmd {
    fn run(self) -> CliResult<()> { unimplemented!() }
}

#[derive(StructOpt, Debug)]
pub(crate) struct LogsList {
    /// User ID.
    #[structopt(name = "id")]
    user_id: Option<u64>,
    #[structopt(name = "start")]
    start_time: Option<u64>,
    #[structopt(name = "end")]
    end_time: Option<u64>,
    #[structopt(name = "limit")]
    limit: Option<u16>,
    #[structopt(name = "skip")]
    offset: Option<u32>,
}

impl CliCommand for LogsList {
    fn run(self) -> CliResult<()> { unimplemented!() }
}


/// returns a `RustyPitchfork` client to use to interact with the Domo API.
fn get_client() -> RustyPitchfork {
    let domo_client_id = env::var("DOMO_CLIENT_ID").unwrap();
    let domo_secret = env::var("DOMO_SECRET").unwrap();
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .client_scope(DomoScope{
            data: false,
            user: false,
            audit: true,
            dashboard: false,
        })
        .build();
    RustyPitchfork::default().auth_manager(client_creds).build()
}