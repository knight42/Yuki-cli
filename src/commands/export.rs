use clap::{App, Arg, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;
use serde_json::{self, Value};
use std::io;

pub(crate) struct Export;

impl Commander for Export {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("export")
            .about("Export config")
            .arg(
                Arg::with_name("pretty")
                    .short("p")
                    .long("pretty")
                    .help("Human-readable format"),
            )
            .arg(
                Arg::with_name("NAMES")
                    .multiple(true)
                    .help("Repository names"),
            )
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let pretty = args.is_present("pretty");
        let api = ctx.remote.join("config")?;
        let names: Vec<_> = args.values_of("NAMES").unwrap_or_default().collect();
        let mut req = ctx.get(api);
        if !names.is_empty() {
            req.query(&[("names", names.join(","))]);
        }
        let mut resp = req.send()?;
        exit_on_error!(resp);

        if pretty {
            let v: Value = resp.json()?;
            serde_json::to_writer_pretty(io::stdout(), &v)?;
        } else {
            resp.copy_to(&mut io::stdout())?;
        }

        Ok(())
    }
}
