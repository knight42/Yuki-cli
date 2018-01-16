use clap::{App, Arg, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;
use serde_json::{self, Value};
use std::io;

pub(crate) struct RepoLs;

impl Commander for RepoLs {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("ls")
            .about("List one or all repositories")
            .arg(Arg::with_name("NAME").help("Repository name"))
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let mut remote = ctx.remote.join("repositories")?;
        let name = args.value_of("NAME");

        if name.is_none() {
            let mut r = ctx.client.get(remote).send()?;
            exit_on_error!(r);
            let repos: Value = r.json()?;
            serde_json::to_writer_pretty(io::stdout(), &repos)?;
            return Ok(());
        }

        let name = name.unwrap();
        remote.path_segments_mut().unwrap().push(name);
        let mut r = ctx.client.get(remote).send()?;
        exit_on_error!(r);
        let repo: Value = r.json()?;
        serde_json::to_writer_pretty(io::stdout(), &repo)?;
        Ok(())
    }
}
