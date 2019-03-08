extern crate docopt;
extern crate rusty_pitchfork;

use rusty_pitchfork::auth::DomoClientAppCredentials;
use rusty_pitchfork::client::RustyPitchfork;

use std::env;

use crate::util;
use crate::CliResult;
use serde::Deserialize;
static USAGE: &'static str = "
Interact with Domo Users API.

When uploading column order will be the same order as the input and will fail
if the order doesn't match the user schema in Domo. The schema can automatically
be updated to match the upload source with the '--update-schema' flag.

Usage:
    ripdomo user add 
    ripdomo user remove [options]
    ripdomo user up [options] <input>
    ripdomo user info [options]
    ripdomo user list [options]
    ripdomo user --help
    ripdomo user -h

user options:
    -d, --user-id <id>   Domo user <id> to upload to.
    -l, --limit <limit>     Limit to return in list users.
    -s, --skip <offset>     Offset to start Domo Dataset List from.

common options:
    -h, --help              Display this message
    -o, --output <file>     Write output to <file> instead of stdout
";

#[derive(Deserialize)]
struct Args {
    cmd_add: bool,
    cmd_remove: bool,
    cmd_info: bool,
    cmd_list: bool,
    flag_limit: Option<u32>,
    flag_skip: Option<u32>,
    flag_user_id: Option<u64>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.cmd_info {
        args.user_info()
    } else if args.cmd_list {
        args.user_list()
    } else if args.cmd_add {
        args.user_add()
    } else if args.cmd_remove {
        args.user_remove()
    } else {
        unreachable!();
    }
}

impl Args {
    /// List users given a limit and number to skip. First 500 if no parameters are given.
    fn user_list(&self) -> CliResult<()> {
        let lim = match &self.flag_limit {
            Some(num) => *num,
            None => 500,
        };

        let skip = match &self.flag_skip {
            Some(num) => *num,
            None => 0,
        };

        let domo = get_client();
        let users = domo.list_users(lim, skip)?;
        println!("{:?}", users);
        Ok(())
    }

    /// Print Info for a given user.
    fn user_info(&self) -> CliResult<()> {
        let user_id = match &self.flag_user_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Dataset Id Given"),
        };

        let domo = get_client();
        let info = domo.user(user_id)?;
        println!("{:?}", info);
        Ok(())
    }

    /// Create a new Domo user.
    /// TODO: implement add new user(s) command
    fn user_add(&self) -> CliResult<()> {
        //println!("{:?}", user);
        // let user = User {
        //     name: "".to_string(),
        //     email: "".to_string(),
        //     role: "".to_string(),
        //     title: "".to_string(),
        //     alternate_email: "".to_string(),
        //     phone: "".to_string(),
        //     location: "".to_string(),
        //     timezone: "".to_string(),
        //     image_uri: "".to_string(),
        //     employee_number: "".to_string(),
        // };
        // let domo = get_client();
        // let new_user = domo.create_user(user)?;
        // println!("User Name: {}", new_user.name);
        println!("Not Yet Implemented");

        Ok(())
    }

    // TODO: create Update user(s) command

    /// Delete a given Domo user.
    fn user_remove(&self) -> CliResult<()> {
        println!("DS remove");
        let user_id = match &self.flag_user_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No User Id Given"),
        };

        let domo = get_client();
        domo.delete_user(user_id)?;
        Ok(())
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
