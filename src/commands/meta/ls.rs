use chrono::{DateTime, Local};
use clap::{App, Arg, ArgMatches, SubCommand};
use commands::{ts_local, Commander};
use context::Context;
use serde::{Deserialize, Deserializer};
use serde_json;
use std::io;

pub(crate) struct MetaLs;

fn pretty_size<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s = i64::deserialize(d)?;
    for unit in ["KiB", "MiB"].iter() {
        if s < 1024 {
            return Ok(format!("{} {}", s, unit));
        } else {
            s /= 1024;
        }
    }
    Ok(format!("{} GiB", s))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meta {
    #[serde(with = "ts_local")] created_at: DateTime<Local>,
    #[serde(with = "ts_local")] updated_at: DateTime<Local>,
    #[serde(with = "ts_local")] last_success: DateTime<Local>,
    #[serde(deserialize_with = "pretty_size")] size: String,
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
            let mut r = ctx.client.get(remote).send()?;
            exit_on_error!(r);
            let repos: Vec<Meta> = r.json()?;
            serde_json::to_writer_pretty(io::stdout(), &repos)?;
            return Ok(());
        }

        let name = name.unwrap();
        remote.path_segments_mut().unwrap().push(name);
        let mut r = ctx.client.get(remote).send()?;
        exit_on_error!(r);
        let repo: Meta = r.json()?;
        serde_json::to_writer_pretty(io::stdout(), &repo)?;
        Ok(())
    }
}
