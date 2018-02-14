use clap::{App, Arg, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;
use std::io;

pub(crate) struct Sync_;

impl Commander for Sync_ {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("sync")
            .about("Sync local repository with remote")
            .arg(
                Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .help("Debug mode"),
            )
            .arg(Arg::with_name("REPO").index(1).required(true))
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let verbose = args.is_present("verbose");
        let repo = args.value_of("REPO").unwrap();
        let mut api = ctx.remote.join(&format!("containers/{}", repo))?;
        let mut req = ctx.post(api.as_str());
        if verbose {
            req.query(&[("debug", "1")]);
        }
        let mut resp = req.send()?;
        exit_on_error!(resp);

        if verbose {
            api.path_segments_mut().unwrap().push("logs");
            let mut resp = ctx.get(api).query(&[("follow", "1")]).send()?;
            exit_on_error!(resp);
            resp.copy_to(&mut io::stdout())?;
        }
        Ok(())
    }
}
