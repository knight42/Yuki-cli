use clap::{App, Arg, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;
use std::io;

pub(crate) struct RepoLogs;

impl Commander for RepoLogs {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("logs")
            .about("Logs")
            .arg(
                Arg::with_name("nth")
                    .short("n")
                    .long("nth")
                    .takes_value(true)
                    .help("View the nth log file"),
            )
            .arg(
                Arg::with_name("tail")
                    .long("tail")
                    .takes_value(true)
                    .help("Output the last N lines"),
            )
            .arg(
                Arg::with_name("NAME")
                    .required(true)
                    .help("Repository name"),
            )
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let name = args.value_of("NAME").unwrap();
        let mut remote = ctx.remote.join("repositories").unwrap();
        remote.path_segments_mut().unwrap().push(name).push("logs");
        if args.is_present("nth") {
            value_t_or_exit!(args, "nth", u8);
            remote
                .query_pairs_mut()
                .append_pair("n", args.value_of("nth").unwrap());
        }
        if args.is_present("tail") {
            value_t_or_exit!(args, "tail", u8);
            remote
                .query_pairs_mut()
                .append_pair("tail", args.value_of("tail").unwrap());
        }
        let mut resp = ctx.client.get(remote).send()?;
        exit_on_error!(resp);
        resp.copy_to(&mut io::stdout())?;
        Ok(())
    }
}
