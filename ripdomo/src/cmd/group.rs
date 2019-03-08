extern crate docopt;
extern crate rusty_pitchfork;

use rusty_pitchfork::auth::DomoClientAppCredentials;
// use rusty_pitchfork::domo::group::*;
use std::env;

use rusty_pitchfork::client::RustyPitchfork;
use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &'static str = "
Interact with Domo Groups API.

When uploading column order will be the same order as the input and will fail
if the order doesn't match the group schema in Domo. The schema can automatically
be updated to match the upload source with the '--update-schema' flag.

Usage:
    ripdomo group add
    ripdomo group remove [options]
    ripdomo group up [options] <input>
    ripdomo group info [options]
    ripdomo group list [options]
    ripdomo group --help
    ripdomo group -h

group options:
    -d, --group-id <id>   Domo group <id> to upload to.
    -l, --limit <limit>     Limit to return in list groups.
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
    flag_group_id: Option<u64>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.cmd_info {
        args.group_info()
    } else if args.cmd_list {
        args.group_list()
    } else if args.cmd_add {
        args.group_add()
    } else if args.cmd_remove {
        args.group_remove()
    } else {
        unreachable!();
    }
}

impl Args {
    /// List groups given a limit and number to skip. First 500 if no parameters are given.
    fn group_list(&self) -> CliResult<()> {
        let lim = match &self.flag_limit {
            Some(num) => *num,
            None => 500,
        };

        let skip = match &self.flag_skip {
            Some(num) => *num,
            None => 0,
        };

        let domo = get_client();
        let groups = domo.list_groups(lim, skip)?;
        println!("{:?}", groups);
        Ok(())
    }

    /// Print Info for a given group.
    fn group_info(&self) -> CliResult<()> {
        let group_id = match &self.flag_group_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Dataset Id Given"),
        };

        let domo = get_client();
        let info = domo.group(group_id)?;
        println!("{:?}", info);
        Ok(())
    }

    /// Create a new Domo group.
    /// TODO: implement add new group(s) command
    fn group_add(&self) -> CliResult<()> {
        //println!("{:?}", group);
        // let group = Group {
        //     name: "".to_string(),
        //     id: 0u64,
        // };
        // let domo = get_client();
        // let new_group = domo.create_group(group)?;
        // println!("Group Name: {}", new_group.name);
        println!("Not Yet Implemented");

        Ok(())
    }

    // TODO: create Update group(s) command

    /// Delete a given Domo group.
    fn group_remove(&self) -> CliResult<()> {
        println!("DS remove");
        let group_id = match &self.flag_group_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Group Id Given"),
        };

        let domo = get_client();
        domo.delete_group(group_id)?;
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
