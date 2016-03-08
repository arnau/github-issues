extern crate csv;
extern crate regex;
extern crate curl;
extern crate docopt;
extern crate rustc_serialize;

#[macro_use]
extern crate log;

// use std::env;
// use std::fmt;
use std::process;
use docopt::Docopt;

mod cmd;


macro_rules! check_repopath {
    ($path:expr) => (
        if $path.len() != 2 {
            println!("<repopath> must have the form <owner>/<repo>.  e.g. ustwo/github-issues");
            process::exit(1)
        }
    );
}

// github-issues fetch <repopath> --oauth-token=<oauth_token> --csv --output=<file> [--label=<label>...]
const USAGE: &'static str = "
Github issue consumer.

Usage:
    github-issues fetch <repopath> --oauth-token=<oauth_token> --csv --output=<file> [--label=<label>...]
    github-issues --version
    github-issues (-h | --help)

Options:
    -h, --help                        Display this message
    --version                         Display the current version
    --oauth-token=<oauth_token>       Github OAuth authorisation token
    --csv                             Output CSV
    --output=<file>                   File where to store the data
    --label=<label>                   Github label to filter with
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_version: bool,
    cmd_fetch: bool,
    arg_repopath: String,
    flag_label: Vec<String>,
    flag_oauth_token: String,
    flag_csv: bool,
    flag_output: String,
}

// #[derive(Debug, RustcDecodable)]
// enum Command {
//     Fetch
// }

// impl Command {
//     fn run(self) -> CliResult<()> {
//         println!("foo");
//     }
// }

pub fn version() -> String {
    let (maj, min, pat) = (option_env!("CARGO_PKG_VERSION_MAJOR"),
                           option_env!("CARGO_PKG_VERSION_MINOR"),
                           option_env!("CARGO_PKG_VERSION_PATCH"));
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) =>
            format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("github-issues version {}", version());
        return;
    }

    if args.cmd_fetch {
        let repopath: Vec<&str> = args.arg_repopath.split("/").collect();

        check_repopath!(repopath);

        cmd::fetch::run(repopath[0],
                        repopath[1],
                        args.flag_oauth_token,
                        args.flag_label,
                        args.flag_output);
    }
}
