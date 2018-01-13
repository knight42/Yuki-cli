use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::Response;
use commands::{Commander, RespMsg};
use context::Context;

pub(crate) struct RepoRm;

impl Commander for RepoRm {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("rm")
            .about("Remove one or more repositories")
            .arg(
                Arg::with_name("REPOS")
                    .multiple(true)
                    .required(true)
                    .help("Repository names to remove"),
            )
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let remote = ctx.remote.join("repositories/")?;
        let repos: Vec<_> = args.values_of("REPOS").unwrap_or_default().collect();

        let result = repos.iter().map(|repo| -> ::Result<Response> {
            let remote = remote.join(repo)?;
            Ok(ctx.client.delete(remote).send()?)
        });

        for (name, rr) in repos.iter().zip(result) {
            match rr {
                Ok(mut r) => {
                    if !r.status().is_success() {
                        match r.json::<RespMsg>() {
                            Ok(msg) => println!("{} error: {}", name, msg.message),
                            Err(e) => println!("{} error: {}", name, e),
                        }
                    } else {
                        println!("{}", name);
                    }
                },
                Err(e) => println!("{} error: {}", name, e),
            }
        }

        Ok(())
    }
}
