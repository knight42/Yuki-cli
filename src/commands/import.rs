use clap::{App, Arg, ArgMatches, SubCommand};
use commands::Commander;
use context::Context;
use serde_json::{self, Value};
use std::fs::File;

pub(crate) struct Import;

impl Commander for Import {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("import")
            .about("Import config")
            .arg(Arg::with_name("FILE").index(1).required(true))
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let api = ctx.remote.join("config")?;
        let fname = args.value_of("FILE").unwrap();
        let f = File::open(fname)?;
        let v: Value = serde_json::from_reader(f)?;
        let mut resp = ctx.post(api).json(&v).send()?;
        exit_on_error!(resp);
        println!("Successfully imported");
        Ok(())
    }
}
