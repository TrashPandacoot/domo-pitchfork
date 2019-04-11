use crate::CliCommand;
use crate::CliResult;
use domo_pitchfork::auth::DomoClientAppCredentials;
use domo_pitchfork::auth::DomoScope;
use domo_pitchfork::pitchfork::DomoPitchfork;
use std::env;
use log::{trace};
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
pub(crate) enum UserCmd {
    /// List Domo Users
    #[structopt(name = "create")]
    List(UsersList),
    /// Get info for a given User
    #[structopt(name = "info")]
    Info(UserInfo),
    /// Create a new Domo User.
    #[structopt(name = "create")]
    Create(UserCreate),
    /// Delete a Domo User.
    #[structopt(name = "delete")]
    Remove(UserDelete),
    /// Modify a Domo User.
    #[structopt(name = "modify")]
    Modify(UserModify),
}
impl CliCommand for UserCmd {
    fn run(self) -> CliResult<()> {
        match self {
            UserCmd::List(a) => a.run(),
            UserCmd::Info(a) => a.run(),
            UserCmd::Create(a) => a.run(),
            UserCmd::Remove(a) => a.run(),
            UserCmd::Modify(a) => a.run(),
        }
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct UsersList {
    #[structopt(short = "l", long = "limit", default_value = "50")]
    limit: u32,
    #[structopt(short = "s", long = "offset", default_value = "0")]
    offset: u32,
}
impl CliCommand for UsersList {
    fn run(self) -> CliResult<()> {
        let token = token();
        let domo = DomoPitchfork::with_token(&token);
        let users = domo.users().list(self.limit, self.offset)?;
        println!("{:?}", users);
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct UserInfo {
    /// User ID.
    #[structopt(name = "id")]
    user_id: u64,
}
impl CliCommand for UserInfo {
    fn run(self) -> CliResult<()> {
        let token = token();
        let domo = DomoPitchfork::with_token(&token);
        let info = domo.users().info(self.user_id)?;
        println!("{:?}", info);
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct UserCreate {
    /// User ID.
    #[structopt(name = "id")]
    user_id: u64,
}
impl CliCommand for UserCreate {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct UserDelete {
    /// User ID.
    #[structopt(name = "id")]
    user_id: u64,
}
impl CliCommand for UserDelete {
    fn run(self) -> CliResult<()> {
        let token = token();
        let domo = DomoPitchfork::with_token(&token);
        domo.users().delete(self.user_id)?;
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct UserModify {
    /// User ID.
    #[structopt(name = "id")]
    user_id: u64,
}
impl CliCommand for UserModify {
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
            data: false,
            user: true,
            audit: false,
            dashboard: false,
            buzz: false,
            account: false,
            workflow: false,
        })
        .build();
        client_creds.get_access_token()
}
