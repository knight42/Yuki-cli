use chrono::{DateTime, Local};
use clap::{App, Arg, ArgMatches, SubCommand};
use commands::{pretty_size, ts_local, default_date, Commander};
use context::Context;

pub(crate) struct MetaLs;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meta {
    #[serde(with = "ts_local")]
    #[serde(default = "default_date")]
    created_at: DateTime<Local>,
    #[serde(with = "ts_local")]
    #[serde(default = "default_date")]
    updated_at: DateTime<Local>,
    exit_code: i64,
    #[serde(with = "ts_local")]
    #[serde(default = "default_date")]
    last_success: DateTime<Local>,
    #[serde(serialize_with = "pretty_size")] size: i64,
    name: String,
    syncing: bool,
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
            pprint_json!(repos);
            return Ok(());
        }

        let name = name.unwrap();
        remote.path_segments_mut().unwrap().push(name);
        let mut r = ctx.get(remote).send()?;
        exit_on_error!(r);
        let repo: Meta = r.json()?;
        pprint_json!(repo);
        Ok(())
    }
}
