use crate::CliCommand;
use crate::CliResult;
use rusty_pitchfork::auth::DomoClientAppCredentials;
use rusty_pitchfork::auth::DomoScope;
use rusty_pitchfork::client::RustyPitchfork;
use std::env;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
pub(crate) enum PageCmd {
    #[structopt(name = "list")]
    List(PageList),
    #[structopt(name = "info")]
    Info(PageInfo),
    #[structopt(name = "create")]
    Create(PageCreate),
    #[structopt(name = "modify")]
    Modify(PageModify),
    #[structopt(name = "collection")]
    Collection(PageCollection),
    #[structopt(name = "delete")]
    Remove(PageRemove),
}
impl CliCommand for PageCmd {
    fn run(self) -> CliResult<()> {
        match self {
            PageCmd::List(a) => a.run(),
            PageCmd::Info(a) => a.run(),
            PageCmd::Create(a) => a.run(),
            PageCmd::Remove(a) => a.run(),
            PageCmd::Modify(a) => a.run(),
            PageCmd::Collection(a) => a.run(),
        }
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct PageList {}
impl CliCommand for PageList {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct PageInfo {}
impl CliCommand for PageInfo {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct PageCreate {}
impl CliCommand for PageCreate {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct PageModify {}
impl CliCommand for PageModify {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct PageRemove {}
impl CliCommand for PageRemove {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}

#[derive(StructOpt, Debug)]
pub(crate) enum PageCollection {
    #[structopt(name = "info")]
    Info(PageCollectionInfo),
    #[structopt(name = "create")]
    Create(PageCollectionCreate),
    #[structopt(name = "modify")]
    Modify(PageCollectionModify),
    #[structopt(name = "delete")]
    Remove(PageCollectionRemove),
}
impl CliCommand for PageCollection {
    fn run(self) -> CliResult<()> {
        match self {
            PageCollection::Info(a) => a.run(),
            PageCollection::Create(a) => a.run(),
            PageCollection::Remove(a) => a.run(),
            PageCollection::Modify(a) => a.run(),
        }
    }
}

#[derive(StructOpt, Debug)]
pub(crate) struct PageCollectionInfo {}
impl CliCommand for PageCollectionInfo {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct PageCollectionCreate {}
impl CliCommand for PageCollectionCreate {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct PageCollectionModify {}
impl CliCommand for PageCollectionModify {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
#[derive(StructOpt, Debug)]
pub(crate) struct PageCollectionRemove {}
impl CliCommand for PageCollectionRemove {
    fn run(self) -> CliResult<()> {
        unimplemented!()
    }
}
// impl Args {
//     /// List pages given a limit and number to skip. First 500 if no parameters are given.
//     fn page_list(&self) -> CliResult<()> {
//         let lim = match &self.flag_limit {
//             Some(num) => *num,
//             None => 500,
//         };

//         let skip = match &self.flag_skip {
//             Some(num) => *num,
//             None => 0,
//         };

//         let domo = get_client();
//         let pages = domo.list_pages(lim, skip)?;
//         println!("{:?}", pages);
//         Ok(())
//     }

//     /// Print Info for a given page.
//     fn page_info(&self) -> CliResult<()> {
//         let page_id = match &self.flag_page_id {
//             Some(id) => id.to_owned(),
//             _ => return fail!("No Dataset Id Given"),
//         };

//         let domo = get_client();
//         let info = domo.page(page_id)?;
//         println!("{:?}", info);
//         Ok(())
//     }

//     /// Create a new Domo page.
//     /// TODO: implement add new page(s) command
//     fn page_add(&self) -> CliResult<()> {
//         //println!("{:?}", page);
//         // let page = Page {
//         //     name: "".to_string(),
//         //     id: 0u64,
//         // };
//         // let domo = get_client();
//         // let new_page = domo.create_page(page)?;
//         // println!("Page Name: {}", new_page.name);
//         println!("Not Yet Implemented");

//         Ok(())
//     }

//     // TODO: create Update page(s) command

//     /// Delete a given Domo page.
//     fn page_remove(&self) -> CliResult<()> {
//         println!("DS remove");
//         let page_id = match &self.flag_page_id {
//             Some(id) => id.to_owned(),
//             _ => return fail!("No Page Id Given"),
//         };

//         let domo = get_client();
//         domo.delete_page(page_id)?;
//         Ok(())
//     }
// }

/// returns a `RustyPitchfork` client to use to interact with the Domo API.
fn get_client() -> RustyPitchfork {
    let domo_client_id = env::var("DOMO_CLIENT_ID").unwrap();
    let domo_secret = env::var("DOMO_SECRET").unwrap();
    let client_creds = DomoClientAppCredentials::default()
        .client_id(&domo_client_id)
        .client_secret(&domo_secret)
        .client_scope(DomoScope {
            data: false,
            user: false,
            audit: false,
            dashboard: true,
        })
        .build();
    RustyPitchfork::default().auth_manager(client_creds).build()
}
