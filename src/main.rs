extern crate chrono;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub(crate) type Result<T> = ::std::result::Result<T, failure::Error>;

mod commands;
mod context;

use clap::{App, AppSettings};
use commands::Commander;

fn main() {
    let matches = build_cli().get_matches();
    let builder = context::Context::builder();
    let ctx = builder.build().unwrap();

    let result = match matches.subcommand() {
        ("ct", Some(args)) => commands::Ct::exec(&ctx, args),
        ("meta", Some(args)) => commands::Meta::exec(&ctx, args),
        ("repo", Some(args)) => commands::Repo::exec(&ctx, args),
        ("sync", Some(args)) => commands::Sync_::exec(&ctx, args),
        ("completion", Some(args)) => commands::Completion::exec(&ctx, args),

        //("login", Some(args)) => (),
        // ("logout", Some(args)) => (),
        ("import", Some(args)) => commands::Import::exec(&ctx, args),
        ("export", Some(args)) => commands::Export::exec(&ctx, args),
        (c, _) => Err(format_err!("unknown command: {}", c)),
    };
    if let Err(e) = result {
        println!("Error: {}", e);
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
        .subcommand(commands::Completion::build())
        .subcommand(commands::Repo::build())
        .subcommand(commands::Sync_::build())
        .subcommand(commands::Meta::build())
        .subcommand(commands::Export::build())
        .subcommand(commands::Import::build())
        .subcommand(commands::Ct::build())
}
