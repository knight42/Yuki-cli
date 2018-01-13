use ::clap::{App, Arg, ArgMatches, SubCommand};
use ::context::Context;
use ::commands::{RespMsg,Commander};
use std::io;

pub struct Sync_;

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
        if !resp.status().is_success() {
            let r: RespMsg = resp.json()?;
            return Err(format_err!("{}", r.message));
        }

        if verbose {
            api.set_query(None);
            api.path_segments_mut().unwrap().push("logs");
            api.set_query(Some("follow=1"));
            let mut resp = ctx.client.get(api).send()?;
            let stdout = io::stdout();
            let mut handler = stdout.lock();
            resp.copy_to(&mut handler).ok();
        }
        Ok(())
    }
}
