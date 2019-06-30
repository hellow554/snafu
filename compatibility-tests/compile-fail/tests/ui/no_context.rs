extern crate snafu;

use snafu::Snafu;

type BoxError = Box<std::error::Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(context(false))]
    MissingSource { },

    #[snafu(context(false))]
    HasUserFields { source: BoxError, a: i32, b: i32 },
}

fn main() {}
