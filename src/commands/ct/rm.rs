use clap::{App, Arg, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;

pub(crate) struct CtRm;

impl Commander for CtRm {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("ls")
            .about("Remove the given container")
            .arg(Arg::with_name("NAME"))
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let mut remote = ctx.remote.join("containers")?;
        let name = args.value_of("NAME").unwrap();
        remote.path_segments_mut().unwrap().push(name);
        let mut r = ctx.delete(remote).send()?;
        exit_on_error!(r);
        println!("Removed");
        Ok(())
    }
}
