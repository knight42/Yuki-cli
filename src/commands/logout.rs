use clap::{App, ArgMatches, SubCommand};
use context::Context;
use std::fs;

pub(crate) struct Logout;

impl super::Commander for Logout {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("logout").about("Log out")
    }
    fn exec(ctx: &Context, _: &ArgMatches) -> ::Result<()> {
        let f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(ctx.homedir.join("token"))?;
        f.set_len(0)?;
        let remote = ctx.remote.join("sessions")?;
        let mut r = ctx.delete(remote).send()?;
        exit_on_error!(r);
        println!("Logout successfully.");
        Ok(())
    }
}
