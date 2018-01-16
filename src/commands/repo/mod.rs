use clap::{App, AppSettings, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;

mod rm;
mod ls;
mod logs;

use self::logs::RepoLogs;
use self::ls::RepoLs;
use self::rm::RepoRm;

pub(crate) struct Repo;

impl Commander for Repo {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("repo")
            .about("Manage repositories")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            //.subcommand(SubCommand::with_name("update").about(""))
            .subcommand(RepoRm::build())
            .subcommand(RepoLs::build())
            .subcommand(RepoLogs::build())
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        match args.subcommand() {
            ("rm", Some(args)) => RepoRm::exec(ctx, args),
            ("ls", Some(args)) => RepoLs::exec(ctx, args),
            ("logs", Some(args)) => RepoLogs::exec(ctx, args),
            (c, _) => Err(format_err!("unknown command: {}", c)),
        }
    }
}
