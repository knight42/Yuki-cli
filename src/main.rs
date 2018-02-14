extern crate chrono;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate rpassword;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub(crate) type Result<T> = ::std::result::Result<T, failure::Error>;

mod commands;
mod context;

use clap::{App, AppSettings, Arg};
use commands::Commander;
use reqwest::header::{self, Headers};
use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

macro_rules! exit {
    ($fmt:expr) => ({
        eprintln!($fmt);
        ::std::process::exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!($fmt, $($arg)*);
        ::std::process::exit(1);
    });
}

fn find_token<R: Read>(r: R) -> Option<String> {
    let reader = BufReader::new(r);
    reader
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|s| {
                let l = s.trim();
                if l.is_empty() {
                    None
                } else {
                    Some(l.to_string())
                }
            })
        })
        .next()
}

fn main() {
    let matches = build_cli().get_matches();
    let homedir = match env::home_dir() {
        Some(mut d) => {
            d.push(".yuki");
            d
        }
        None => exit!("Impossible to get your home dir"),
    };

    if let Err(e) = fs::create_dir_all(&homedir) {
        exit!("Cannot create {}: {}", homedir.display(), e);
    }

    let mut builder = context::Context::builder();

    let remote = matches.value_of("remote").unwrap();
    builder.set_remote(remote);

    let f = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(homedir.join("token"))
        .expect("Cannot read token file");

    if let Some(token) = find_token(f) {
        let mut h = Headers::new();
        h.set(header::Authorization(header::Bearer { token }));
        builder.set_headers(h);
    }

    builder.set_homedir(&homedir);

    let ctx = match builder.build() {
        Ok(b) => b,
        Err(e) => exit!("{}", e),
    };

    let result = match matches.subcommand() {
        ("ct", Some(args)) => commands::Ct::exec(&ctx, args),
        ("meta", Some(args)) => commands::Meta::exec(&ctx, args),
        ("repo", Some(args)) => commands::Repo::exec(&ctx, args),
        ("sync", Some(args)) => commands::Sync_::exec(&ctx, args),
        ("login", Some(args)) => commands::Login::exec(&ctx, args),
        ("logout", Some(args)) => commands::Logout::exec(&ctx, args),
        ("import", Some(args)) => commands::Import::exec(&ctx, args),
        ("export", Some(args)) => commands::Export::exec(&ctx, args),
        ("completion", Some(args)) => commands::Completion::exec(&ctx, args),
        (c, _) => Err(format_err!("unknown command: {}", c)),
    };
    if let Err(e) = result {
        exit!("{}", e)
    }
}

pub(crate) fn build_cli() -> App<'static, 'static> {
    App::new("yuki")
        .bin_name("yuki")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .after_help(
            "You can also run `yuki SUBCOMMAND -h` to get more information about that subcommand.",
        )
        .arg(
            Arg::with_name("remote")
                .short("r")
                .long("remote")
                .global(true)
                .help("remote registry")
                .default_value(context::DEFAULT_REMOTE),
        )
        .subcommand(commands::Ct::build())
        .subcommand(commands::Repo::build())
        .subcommand(commands::Meta::build())
        .subcommand(commands::Sync_::build())
        .subcommand(commands::Export::build())
        .subcommand(commands::Import::build())
        .subcommand(commands::Login::build())
        .subcommand(commands::Logout::build())
        .subcommand(commands::Completion::build())
}
