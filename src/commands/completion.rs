use build_cli;
use clap::{App, Arg, ArgMatches, Shell, SubCommand};
use context::Context;
use std::io::stdout;

pub struct Completion;

impl super::Commander for Completion {
    fn build() -> App<'static, 'static> {
        SubCommand::with_name("completion")
            .about("Generate completion script for the command")
            .arg(
                Arg::with_name("SHELL")
                    .index(1)
                    .required(true)
                    .possible_values(&["bash", "zsh"]),
            )
    }
    fn exec(_: &Context, args: &ArgMatches) -> ::Result<()> {
        let s = args.value_of("SHELL").unwrap();
        let mut app = build_cli();
        match s {
            "zsh" => app.gen_completions_to("yuki", Shell::Zsh, &mut stdout()),
            "bash" => app.gen_completions_to("yuki", Shell::Bash, &mut stdout()),
            _ => return Err(format_err!("unknown shell")),
        }
        Ok(())
    }
}
