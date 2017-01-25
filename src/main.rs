extern crate gizmo;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Rocket League replay parser.

Usage:
  gizmo [options] <file>
  gizmo (--help | --version)

Options:
  -h, --help     Show this help.
  -V, --version  Show version.
  -v, --verbose  Verbose output.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
    flag_verbose: bool,
    flag_help: bool,
    flag_version: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .unwrap()
        .version(Some(VERSION.to_string()))
        .help(true)
        .decode()
        .unwrap_or_else(|e| e.exit());

    let game = gizmo::Replay::from_file(&args.arg_file)
        .unwrap()
        .parse()
        .unwrap();
    println!("{:?}", game);
}
