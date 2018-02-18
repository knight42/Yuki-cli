use clap::{App, ArgMatches, SubCommand};
use context::Context;
use rpassword::prompt_password_stdout;
use std::fs;
use std::io::{self, Write};

pub(crate) struct Login;

#[derive(Deserialize)]
struct Token {
    token: String,
}

impl super::Commander for Login {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("login").about("Log in")
    }

    fn exec(ctx: &Context, _: &ArgMatches) -> ::Result<()> {
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
            .open(ctx.homedir.join("token"))?;
        f.write_all(t.token.as_bytes())?;
        f.sync_data()?;
        println!("Login successfully.");
        Ok(())
    }
}
