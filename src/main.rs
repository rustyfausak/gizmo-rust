extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Rocket League replay parser.

Usage:
  rlparse [options] <file>
  rlparse (--help | --version)

Options:
  -h, --help     Show this help.
  -V, --version  Show version.
  -v, --verbose  Verbose output.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: Vec<String>,
    flag_verbose: bool,
    flag_help: bool,
    flag_version: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .unwrap_or_else(|e| e.exit())
        .version(Some(VERSION.to_string()))
        .help(true)
        .decode()
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
}
