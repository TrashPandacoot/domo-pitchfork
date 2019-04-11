use domo_pitchfork::auth::{DomoScope, DomoClientAppCredentials};
use domo_pitchfork::pitchfork::DomoPitchfork;
use crate::CliCommand;
use crate::CliResult;
use structopt::StructOpt;
use std::env;
use log::{trace};
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
    start_time: u64,
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


/// returns a token to use with the `DomoPitchfork` client to use to interact with the Domo API.
fn token() -> String {
    let domo_client_id = env::var("DOMO_CLIENT_ID").unwrap();
    let domo_secret = env::var("DOMO_SECRET").unwrap();
    trace!("Authenticating with Domo as Client ID: {}", domo_client_id);
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .client_scope(DomoScope{
            data: false,
            user: false,
            audit: true,
            dashboard: false,
            buzz: false,
            account: false,
            workflow: false,
        })
        .build();
    client_creds.get_access_token()
}