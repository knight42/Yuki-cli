use clap::{App, Arg, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;

pub(crate) struct CtStop;

impl Commander for CtStop {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("stop")
            .about("Stop the given container")
            .arg(Arg::with_name("NAME").required(true))
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let mut remote = ctx.remote.join("containers")?;
        let name = args.value_of("NAME").unwrap();
        remote.path_segments_mut().unwrap().push(name).push("stop");
        let mut r = ctx.get(remote).send()?;
        exit_on_error!(r);
        println!("Stopped");
        Ok(())
    }
}
