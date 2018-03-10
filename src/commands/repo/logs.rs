use chrono::{DateTime, Local};
use clap::{App, Arg, ArgMatches, SubCommand};
use commands::{pretty_size, ts_local, Commander};
use context::Context;
use serde_json;
use std::io;

pub(crate) struct RepoLogs;

#[derive(Serialize, Deserialize)]
struct LogFileStat {
    name: String,
    #[serde(serialize_with = "pretty_size")] size: i64,
    #[serde(serialize_with = "ts_local::serialize")] mtime: DateTime<Local>,
}

impl Commander for RepoLogs {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("logs")
            .about("Logs")
            .arg(
                Arg::with_name("stats")
                    .long("stats")
                    .conflicts_with_all(&["nth", "tail"])
                    .help("Get the information of log files"),
            )
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
        if args.is_present("stats") {
            let mut r = ctx.get(remote).query(&[("stats", "1")]).send()?;
            exit_on_error!(r);
            let stats: Vec<LogFileStat> = r.json()?;
            serde_json::to_writer_pretty(io::stdout(), &stats)?;
            return Ok(());
        }
        let mut req = ctx.get(remote);
        if args.is_present("nth") {
            value_t_or_exit!(args, "nth", u8);
            req.query(&[("n", args.value_of("nth").unwrap())]);
        }
        if args.is_present("tail") {
            value_t_or_exit!(args, "tail", u8);
            req.query(&[("tail", args.value_of("tail").unwrap())]);;
        }
        let mut r = req.send()?;
        exit_on_error!(r);
        r.copy_to(&mut io::stdout())?;
        Ok(())
    }
}
