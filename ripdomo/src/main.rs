#![warn(clippy::all, clippy::pedantic)]
use std::fmt;
use std::io;
use structopt::StructOpt;
use structopt::clap::Shell;
use structopt::clap::AppSettings;
use env_logger;
use log::LevelFilter;
use crate::cmd::dataset::DatasetCmd;
use crate::cmd::stream::StreamCmd;
use crate::cmd::user::UserCmd;
use crate::cmd::group::GroupsCmd;
use crate::cmd::page::PageCmd;
use crate::cmd::logs::LogsCmd;

/// Create Err with a given Error msg string
macro_rules! fail {
    ($e:expr) => {
        Err(::std::convert::From::from($e))
    };
}

mod cmd;

#[derive(StructOpt, Debug)]
#[structopt(name = "ripdomo")]
#[structopt(raw(global_settings = "&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands]"))]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    #[structopt(raw(global = "true"))]
    pub verbose: u8,
    #[structopt(subcommand)]
    command: DomoCommand,
}

#[derive(StructOpt, Debug)]
enum DomoCommand {
    /// Interact with the Domo Datasets API.
    #[structopt(name = "datasets")]
    Datasets(DatasetCmd),
    /// Interact with the Domo Streams API.
    #[structopt(name = "streams")]
    Streams(StreamCmd),
    /// Interact with the Domo Users API.
    #[structopt(name = "users")]
    Users(UserCmd),
    /// Interact with the Domo Pages API.
    #[structopt(name = "pages")]
    Pages(PageCmd),
    /// Interact with the Domo Groups API.
    #[structopt(name = "groups")]
    Groups(GroupsCmd),
    /// Interact with the Domo Audit Log API.
    #[structopt(name = "logs")]
    Logs(LogsCmd),
}


fn main() -> CliResult<()> {
    // generate `bash` completions in "target" directory.
    Opt::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");
    let opt = Opt::from_args();
    let log_level = match opt.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Trace,
    };
    env_logger::Builder::new().filter(Some("ripdomo"), log_level).init();
    match opt.command {
        DomoCommand::Datasets(d) => d.run(),
        DomoCommand::Streams(s) => s.run(),
        DomoCommand::Users(u) => u.run(),
        DomoCommand::Groups(g) => g.run(),
        DomoCommand::Pages(p) => p.run(),
        DomoCommand::Logs(l) => l.run(),
    }
}

pub(crate) trait CliCommand {
    fn run(self) -> CliResult<()>;
}

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Flag(String),
    Csv(csv::Error),
    Io(io::Error),
    Other(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Flag(ref e) => e.fmt(f),
            CliError::Csv(ref e) => e.fmt(f),
            CliError::Io(ref e) => e.fmt(f),
            CliError::Other(ref s) => f.write_str(&**s),
        }
    }
}

impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> Self {
        if !err.is_io_error() {
            return CliError::Csv(err);
        }
        match err.into_kind() {
            csv::ErrorKind::Io(v) => From::from(v),
            _ => unreachable!(),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::Io(err)
    }
}

impl From<domo_pitchfork::error::DomoError> for CliError {
    fn from(_err: domo_pitchfork::error::DomoError) -> Self {
        CliError::Other("Rusty Pitchfork Err".to_owned())
    }
}

impl From<String> for CliError {
    fn from(err: String) -> Self {
        CliError::Other(err)
    }
}

impl<'a> From<&'a str> for CliError {
    fn from(err: &'a str) -> Self {
        CliError::Other(err.to_owned())
    }
}
