extern crate docopt;
extern crate rusty_pitchfork;

use rusty_pitchfork::auth::DomoClientAppCredentials;
use rusty_pitchfork::client::RustyPitchfork;
// use rusty_pitchfork::domo::page::*;
use std::env;

use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &'static str = "
Interact with Domo Pages API.

When uploading column order will be the same order as the input and will fail
if the order doesn't match the page schema in Domo. The schema can automatically
be updated to match the upload source with the '--update-schema' flag.

Usage:
    ripdomo page add
    ripdomo page remove [options]
    ripdomo page up [options] <input>
    ripdomo page info [options]
    ripdomo page list [options]
    ripdomo page --help
    ripdomo page -h

page options:
    -d, --page-id <id>   Domo page <id> to upload to.
    -l, --limit <limit>     Limit to return in list pages.
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
    flag_page_id: Option<u64>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.cmd_info {
        args.page_info()
    } else if args.cmd_list {
        args.page_list()
    } else if args.cmd_add {
        args.page_add()
    } else if args.cmd_remove {
        args.page_remove()
    } else {
        unreachable!();
    }
}

impl Args {
    /// List pages given a limit and number to skip. First 500 if no parameters are given.
    fn page_list(&self) -> CliResult<()> {
        let lim = match &self.flag_limit {
            Some(num) => *num,
            None => 500,
        };

        let skip = match &self.flag_skip {
            Some(num) => *num,
            None => 0,
        };

        let domo = get_client();
        let pages = domo.list_pages(lim, skip)?;
        println!("{:?}", pages);
        Ok(())
    }

    /// Print Info for a given page.
    fn page_info(&self) -> CliResult<()> {
        let page_id = match &self.flag_page_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Dataset Id Given"),
        };

        let domo = get_client();
        let info = domo.page(page_id)?;
        println!("{:?}", info);
        Ok(())
    }

    /// Create a new Domo page.
    /// TODO: implement add new page(s) command
    fn page_add(&self) -> CliResult<()> {
        //println!("{:?}", page);
        // let page = Page {
        //     name: "".to_string(),
        //     id: 0u64,
        // };
        // let domo = get_client();
        // let new_page = domo.create_page(page)?;
        // println!("Page Name: {}", new_page.name);
        println!("Not Yet Implemented");

        Ok(())
    }

    // TODO: create Update page(s) command

    /// Delete a given Domo page.
    fn page_remove(&self) -> CliResult<()> {
        println!("DS remove");
        let page_id = match &self.flag_page_id {
            Some(id) => id.to_owned(),
            _ => return fail!("No Page Id Given"),
        };

        let domo = get_client();
        domo.delete_page(page_id)?;
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
