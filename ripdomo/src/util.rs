use docopt::Docopt;
use serde::de::DeserializeOwned;

use crate::CliResult;
pub fn get_args<T>(usage: &str, argv: &[&str]) -> CliResult<T>
where
    T: DeserializeOwned,
{
    Docopt::new(usage)
        .and_then(|d| d.argv(argv.iter().cloned()).deserialize())
        .map_err(From::from)
}
