use clap::{App, AppSettings, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;

mod rm;
mod stop;
mod ls;
mod logs;

use self::logs::CtLogs;
use self::ls::CtLs;
use self::rm::CtRm;
use self::stop::CtStop;

pub(crate) struct Ct;

impl Commander for Ct {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("ct")
            .about("Manage containers")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(CtLs::build())
            .subcommand(CtRm::build())
            .subcommand(CtStop::build())
            .subcommand(CtLogs::build())
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        match args.subcommand() {
            ("ls", Some(args)) => CtLs::exec(ctx, args),
            ("rm", Some(args)) => CtRm::exec(ctx, args),
            ("stop", Some(args)) => CtStop::exec(ctx, args),
            ("logs", Some(args)) => CtLogs::exec(ctx, args),
            (c, _) => Err(format_err!("unknown command: {}", c)),
        }
    }
}
