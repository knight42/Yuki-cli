use chrono::{DateTime, Local};
use clap::{App, ArgMatches, SubCommand};
use commands::{ts_local, Commander};
use context::Context;
use serde_json;
use std::io;

pub(crate) struct CtLs;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Container {
    #[serde(with = "ts_local")] created: DateTime<Local>,
    id: String,
    image: String,
    name: String,
    state: String,
    status: String,
}

impl Commander for CtLs {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("ls").about("List all containers")
    }

    fn exec(ctx: &Context, _args: &ArgMatches) -> ::Result<()> {
        let remote = ctx.remote.join("containers")?;
        let mut r = ctx.get(remote).send()?;
        exit_on_error!(r);
        let cts: Vec<Container> = r.json()?;
        serde_json::to_writer_pretty(io::stdout(), &cts)?;
        Ok(())
    }
}
