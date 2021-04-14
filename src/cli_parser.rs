use std::{path::PathBuf, str::FromStr, time::Duration};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub(crate) struct Opt {
    /// Number of threads
    #[structopt(short, long, default_value = "2")]
    pub thread_count: usize,

    /// Shared file. Created if doesn't exist.
    #[structopt(short, long, parse(from_os_str), default_value = "/tmp/shm")]
    pub file_path: PathBuf,

    /// Delay in releasing lock. (in secs)
    #[structopt(short, long, parse(try_from_str = parse_seconds), default_value = "0.5")]
    pub seconds: Duration,
}

/// [`structopt::StructOpt`] needs a type to implement [`std::str::FromStr`] trait
/// to be convertible using `parse(try_from_str)` attribute. When
/// `FromStr` impl is not avaiable, a separate function can be provided
/// providing a similar interface like `FromStr` trait.
fn parse_seconds(src: &str) -> Result<Duration, <f64 as FromStr>::Err> {
    let seconds = src.parse::<f64>()?;
    Ok(Duration::from_secs_f64(seconds))
}
