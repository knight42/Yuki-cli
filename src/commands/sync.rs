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
        if verbose {
            api.set_query(Some("debug=1"));
        }
        let mut resp = ctx.client.post(api.as_str()).send()?;
        exit_on_error!(resp);

        if verbose {
            api.set_query(None);
            api.path_segments_mut().unwrap().push("logs");
            api.set_query(Some("follow=1"));
            let mut resp = ctx.client.get(api).send()?;
            exit_on_error!(resp);
            resp.copy_to(&mut io::stdout())?;
        }
        Ok(())
    }
}
