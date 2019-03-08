#![warn(clippy::all, clippy::pedantic)]
extern crate csv;
extern crate docopt;
extern crate rusty_pitchfork;
extern crate serde;
use docopt::Docopt;
use serde::Deserialize;
use std::env;
use std::fmt;
use std::io;
use std::process;

/// Write error to stdout
macro_rules! werr {
    ($($arg:tt)*) => ({
        use std::io::Write;
        (writeln!(&mut ::std::io::stdout(), $($arg)*)).unwrap();
    });
}

/// Create Err with a given Error msg string
macro_rules! fail {
    ($e:expr) => {
        Err(::std::convert::From::from($e))
    };
}

mod cmd;
mod util;

// Write the Docopt usage string.
const USAGE: &str = "
Usage:
    ripdomo <command> [<args>...]
    ripdomo [options]

Options:
    -h, --help      Display this message
    <command> -h    Display the command help Message

Commands:
    dataset         Interact with Datasets
    stream          Interact with Dataset Streams
    user            Interact with Domo Users
    group           Interact with Domo Groups
    page            Interact with Domo Pages
";

#[derive(Deserialize)]
struct Args {
    arg_command: Option<Command>,
}

/// Type of Object to run commands for
#[derive(Debug, Deserialize)]
enum Command {
    Dataset,
    Stream,
    User,
    Group,
    Page,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.options_first(true).deserialize())
        .unwrap_or_else(|e| e.exit());
    if let Some(cmd) = args.arg_command {
        match cmd.run() {
            Ok(()) => process::exit(0),
            Err(CliError::Flag(err)) => err.exit(),
            Err(CliError::Csv(err)) => {
                werr!("{}", err);
                process::exit(1);
            }
            Err(CliError::Io(ref err)) if err.kind() == io::ErrorKind::BrokenPipe => {
                process::exit(0);
            }
            Err(CliError::Io(err)) => {
                werr!("{}", err);
                process::exit(1);
            }
            Err(CliError::Other(msg)) => {
                werr!("{}", msg);
                process::exit(1);
            }
        }
    } else {
        process::exit(1);
    }
}

impl Command {
    fn run(self) -> CliResult<()> {
        let argv: Vec<_> = env::args().map(|v| v.to_owned()).collect();
        let argv: Vec<_> = argv.iter().map(|s| &**s).collect();
        let argv = &*argv;
        match self {
            Command::Dataset => cmd::dataset::run(argv),
            Command::Stream => cmd::stream::run(argv),
            Command::User => cmd::user::run(argv),
            Command::Page => cmd::page::run(argv),
            Command::Group => cmd::group::run(argv),
            // _ => process::exit(1),
        }
    }
}

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Flag(docopt::Error),
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

impl From<docopt::Error> for CliError {
    fn from(err: docopt::Error) -> Self {
        CliError::Flag(err)
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

impl From<rusty_pitchfork::error::DomoError> for CliError {
    fn from(_err: rusty_pitchfork::error::DomoError) -> Self {
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
