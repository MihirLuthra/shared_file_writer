mod cli_parser;

use cli_parser::Opt;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

}
