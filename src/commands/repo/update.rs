use clap::{App, Arg, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;
use serde_json;

pub(crate) struct RepoUpdate;

impl Commander for RepoUpdate {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("update")
            .about("Update config of given repository")
            .arg(
                Arg::with_name("NAME")
                    .help("Repository name")
                    .index(1)
                    .required(true),
            )
            .arg(
                Arg::with_name("keyvals")
                    .multiple(true)
                    .empty_values(false)
                    .validator(|v: String| -> Result<(), String> {
                        if v.contains("=") {
                            return Ok(());
                        }
                        Err("`=` is not included in the key-value".into())
                    })
                    .required(true)
                    .help("Key-values in the 'key=value' format"),
            )
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let name = args.value_of("NAME").unwrap();
        let kvs = args.values_of("keyvals").unwrap_or_default();
        let remote = ctx.remote.join(&format!("repositories/{}", name)).unwrap();
        let mut set = serde_json::Map::new();
        let mut unset = serde_json::Map::new();
        for kv in kvs {
            let l: Vec<_> = kv.splitn(2, "=").collect();
            if l[1].is_empty() {
                unset.insert(l[0].into(), json!(l[1]));
            } else {
                set.insert(l[0].into(), json!(l[1]));
            }
        }
        let mut body = json!({});
        if !set.is_empty() {
            body["$set"] = json!(set);
        }
        if !unset.is_empty() {
            body["$unset"] = json!(unset);
        }
        let mut resp = ctx.put(remote).json(&body).send()?;
        exit_on_error!(resp);
        Ok(())
    }
}
