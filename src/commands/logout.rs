use serde_json;
use std::io::{Read, Seek, SeekFrom};
use clap::{App, ArgMatches, SubCommand};
use context::Context;
use std::fs;

use token::UserCredential;

pub(crate) struct Logout;

impl super::Commander for Logout {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("logout").about("Log out")
    }
    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(ctx.homedir.join("token"))?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        if buffer.trim() == "" {
            buffer = String::from("[]");
        }

        let host = args.value_of("remote").unwrap();
        let us = match serde_json::from_slice::<Vec<UserCredential>>(buffer.as_bytes()) {
            Ok(v) => v,
            Err(_) => vec![],
        }.into_iter()
            .filter(|u| u.host != host)
            .collect::<Vec<_>>();
        f.seek(SeekFrom::Start(0))?;
        f.set_len(0)?;
        serde_json::to_writer_pretty(f, &us)?;

        let remote = ctx.remote.join("sessions")?;
        let mut r = ctx.delete(remote).send()?;
        exit_on_error!(r);
        println!("Logout successfully.");
        Ok(())
    }
}
