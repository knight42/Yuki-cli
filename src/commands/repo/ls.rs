use chrono::{DateTime, Local};
use clap::{App, Arg, ArgMatches, SubCommand};
use commands::{ts_local, default_date, Commander};
use context::Context;
use serde_json::Value;

pub(crate) struct RepoLs;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Repo {
    #[serde(with = "ts_local")]
    #[serde(default = "default_date")]
    created_at: DateTime<Local>,
    #[serde(with = "ts_local")]
    #[serde(default = "default_date")]
    updated_at: DateTime<Local>,
    name: String,
    image: String,
    interval: String,
    storage_dir: String,
    #[serde(rename = "bindIP")]
    bind_ip: String,
    log_rot_cycle: u8,
    retry: u8,
    user: String,
    envs: Value,
    volumes: Value,
}

#[derive(Serialize, Deserialize)]
struct RepoSummary {
    name: String,
    image: String,
    interval: String,
}

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
            let mut r = ctx.get(remote).send()?;
            exit_on_error!(r);
            let repos: Vec<RepoSummary> = r.json()?;
            pprint_json!(repos);
            return Ok(());
        }

        let name = name.unwrap();
        remote.path_segments_mut().unwrap().push(name);
        let mut r = ctx.get(remote).send()?;
        exit_on_error!(r);
        let repo: Repo = r.json()?;
        pprint_json!(repo);
        Ok(())
    }
}
