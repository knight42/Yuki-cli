use clap::{App, AppSettings, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;

mod ls;

use self::ls::MetaLs;

pub(crate) struct Meta;

impl Commander for Meta {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("meta")
            .about("List metas")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(MetaLs::build())
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        match args.subcommand() {
            ("ls", Some(args)) => MetaLs::exec(ctx, args),
            (c, _) => Err(format_err!("unknown command: {}", c)),
        }
    }
}
