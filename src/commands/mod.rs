mod repo;
mod sync;
mod completion;

pub trait Commander {
    fn build() -> ::App<'static, 'static>;
    fn exec(ctx: &::context::Context, args: &::clap::ArgMatches) -> ::Result<()>;
}

#[derive(Deserialize)]
pub struct RespMsg {
    message: String
}

pub(crate) use self::sync::Sync_;
pub(crate) use self::repo::Repo;
pub(crate) use self::completion::Completion;
