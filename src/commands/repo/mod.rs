use ::clap::{App, ArgMatches, AppSettings, SubCommand};
use ::context::Context;
use ::commands::Commander;

mod rm;
use self::rm::RepoRm;

pub(crate) struct Repo;

impl Commander for Repo {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("repo")
            .about("Manage repositories")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            //.subcommand(SubCommand::with_name("ls").about("List repositories"))
            //.subcommand(SubCommand::with_name("update").about(""))
            //.subcommand(SubCommand::with_name("logs").about(""))
            .subcommand(RepoRm::build())
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        match args.subcommand() {
            ("rm", Some(args)) => RepoRm::exec(ctx, args),
            (c, _) => Err(format_err!("unknown command: {}", c))
        }
    }
}
