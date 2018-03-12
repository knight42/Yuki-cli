use serde_json;
use clap::{App, ArgMatches, SubCommand};
use context::Context;
use rpassword::prompt_password_stdout;
use std::fs;
use std::io::{self, Read, Write, Seek, SeekFrom};

use token::UserCredential;

pub(crate) struct Login;

#[derive(Deserialize)]
struct Token {
    pub token: String,
}

impl super::Commander for Login {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("login").about("Log in")
    }

    fn exec(ctx: &Context, args: &ArgMatches) -> ::Result<()> {
        let remote = ctx.remote.join("sessions")?;
        print!("Username: ");
        io::stdout().flush()?;
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        let username = username.trim();
        let password = prompt_password_stdout("Password: ")?;
        let mut r = ctx.post(remote)
            .basic_auth(username, Some(password))
            .send()?;
        exit_on_error!(r);

        let t: Token = r.json()?;
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

        let mut us: Vec<UserCredential> = match serde_json::from_slice(buffer.as_bytes()) {
            Ok(v) => v,
            Err(_) => vec![],
        };
        let host = args.value_of("remote").unwrap();
        let mut changed = false;
        for u in us.iter_mut() {
            if u.host == host {
                changed = true;
                (*u).token = &t.token;
            }
        }
        if !changed {
            us.push(UserCredential {
                host: &host,
                token: &t.token,
            })
        }

        f.seek(SeekFrom::Start(0))?;
        f.set_len(0)?;
        serde_json::to_writer_pretty(f, &us)?;
        println!("Login successfully.");
        Ok(())
    }
}
