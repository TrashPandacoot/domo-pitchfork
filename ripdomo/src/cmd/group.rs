use domo_pitchfork::auth::DomoScope;
use domo_pitchfork::pitchfork::DomoPitchfork;
use domo_pitchfork::auth::DomoClientAppCredentials;
use std::env;
use crate::CliCommand;
use crate::CliResult;
use structopt::StructOpt;
use log::{trace};
#[derive(StructOpt, Debug)]
pub(crate) enum GroupsCmd {
    #[structopt(name = "list")]
    List(GroupsList),
    #[structopt(name = "info")]
    Info(GroupInfo),
    #[structopt(name = "create")]
    Create(GroupCreate),
    #[structopt(name = "delete")]
    Remove(GroupRemove),
    #[structopt(name = "modify")]
    Modify(GroupModify),
}
impl CliCommand for GroupsCmd {
    fn run(self) -> CliResult<()> {
        match self {
            GroupsCmd::List(a) => a.run(),
            GroupsCmd::Info(a) => a.run(),
            GroupsCmd::Create(a) => a.run(),
            GroupsCmd::Remove(a) => a.run(),
            GroupsCmd::Modify(a) => a.run(),
        }
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct GroupsList {

}
impl CliCommand for GroupsList {
    fn run(self) -> CliResult<()> { unimplemented!() }
}
#[derive(StructOpt, Debug)]
pub(crate) struct GroupInfo {

}
impl CliCommand for GroupInfo {
    fn run(self) -> CliResult<()> { unimplemented!() }
}
#[derive(StructOpt, Debug)]
pub(crate) struct GroupCreate {

}
impl CliCommand for GroupCreate {
    fn run(self) -> CliResult<()> { unimplemented!() }
}
#[derive(StructOpt, Debug)]
pub(crate) struct GroupModify {

}
impl CliCommand for GroupModify {
    fn run(self) -> CliResult<()> { unimplemented!() }
}
#[derive(StructOpt, Debug)]
pub(crate) struct GroupRemove {

}
impl CliCommand for GroupRemove {
    fn run(self) -> CliResult<()> { unimplemented!() }
}

// impl Args {
//     /// List groups given a limit and number to skip. First 500 if no parameters are given.
//     fn group_list(&self) -> CliResult<()> {
//         let lim = match &self.flag_limit {
//             Some(num) => *num,
//             None => 500,
//         };

//         let skip = match &self.flag_skip {
//             Some(num) => *num,
//             None => 0,
//         };

//         let domo = DomoPitchfork::with_token(&token);
//         let groups = domo.list_groups(lim, skip)?;
//         println!("{:?}", groups);
//         Ok(())
//     }

//     /// Print Info for a given group.
//     fn group_info(&self) -> CliResult<()> {
//         let group_id = match &self.flag_group_id {
//             Some(id) => id.to_owned(),
//             _ => return fail!("No Dataset Id Given"),
//         };

//         let domo = DomoPitchfork::with_token(&token);
//         let info = domo.group(group_id)?;
//         println!("{:?}", info);
//         Ok(())
//     }

//     /// Create a new Domo group.
//     /// TODO: implement add new group(s) command
//     fn group_add(&self) -> CliResult<()> {
//         //println!("{:?}", group);
//         // let group = Group {
//         //     name: "".to_string(),
//         //     id: 0u64,
//         // };
//         // let domo = DomoPitchfork::with_token(&token);
//         // let new_group = domo.create_group(group)?;
//         // println!("Group Name: {}", new_group.name);
//         println!("Not Yet Implemented");

//         Ok(())
//     }

//     // TODO: create Update group(s) command

//     /// Delete a given Domo group.
//     fn group_remove(&self) -> CliResult<()> {
//         println!("DS remove");
//         let group_id = match &self.flag_group_id {
//             Some(id) => id.to_owned(),
//             _ => return fail!("No Group Id Given"),
//         };

//         let domo = DomoPitchfork::with_token(&token);
//         domo.delete_group(group_id)?;
//         Ok(())
//     }
// }

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
