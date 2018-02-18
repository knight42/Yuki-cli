use chrono::{DateTime, Local, TimeZone};
use clap::{App, Arg, ArgMatches, SubCommand};
use commands::{pretty_size, ts_local, Commander};
use context::Context;
use serde_json;
use std::io;

pub(crate) struct MetaLs;

fn default_date() -> DateTime<Local> {
    return Local.timestamp(0, 0);
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meta {
    #[serde(with = "ts_local")]
    #[serde(default = "default_date")]
    created_at: DateTime<Local>,
    #[serde(with = "ts_local")]
    #[serde(default = "default_date")]
    updated_at: DateTime<Local>,
    #[serde(with = "ts_local")]
    #[serde(default = "default_date")]
    last_success: DateTime<Local>,
    #[serde(serialize_with = "pretty_size")] size: i64,
    name: String,
    upstream: String,
}

impl Commander for MetaLs {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("ls")
            .about("List one or all meta")
            .arg(Arg::with_name("NAME").help("Repository name"))
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let mut remote = ctx.remote.join("metas")?;
        let name = args.value_of("NAME");

        if name.is_none() {
            let mut r = ctx.get(remote).send()?;
            exit_on_error!(r);
            let repos: Vec<Meta> = r.json()?;
            serde_json::to_writer_pretty(io::stdout(), &repos)?;
            return Ok(());
        }

        let name = name.unwrap();
        remote.path_segments_mut().unwrap().push(name);
        let mut r = ctx.get(remote).send()?;
        exit_on_error!(r);
        let repo: Meta = r.json()?;
        serde_json::to_writer_pretty(io::stdout(), &repo)?;
        Ok(())
    }
}
