use serde::Serializer;
use chrono::{DateTime, Local, TimeZone};

pub(crate) fn default_date() -> DateTime<Local> {
    Local.timestamp(0, 0)
}

macro_rules! exit_on_error {
    ($r:ident) => ({
        if !$r.status().is_success() {
            let rm: $crate::commands::RespMsg = $r.json()?;
            return Err($crate::failure::err_msg(rm.message));
        }
    })
}

macro_rules! pprint_json {
    ($v:ident) => ({
        $crate::serde_json::to_writer_pretty(::std::io::stdout(), &$v)?;
        println!();
    })
}

pub(crate) fn pretty_size<S>(size: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut size = *size as f64;
    for unit in &["B", "KiB", "MiB", "GiB"] {
        if size < 1024_f64 {
            return serializer.serialize_str(&format!("{:.1} {}", size, unit));
        } else {
            size /= 1024_f64;
        }
    }
    serializer.serialize_str(&format!("{:.1} TiB", size))
}

pub(crate) mod ts_local {
    use chrono::{DateTime, Local};
    use chrono::serde::ts_seconds::deserialize as from_ts;
    use serde::{Deserializer, Serializer};
    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        from_ts(d).map(|dt| dt.with_timezone(&Local))
    }
}

pub trait Commander {
    fn build() -> ::App<'static, 'static>;
    fn exec(ctx: &::context::Context, args: &::clap::ArgMatches) -> ::Result<()>;
}

#[derive(Deserialize)]
pub(crate) struct RespMsg {
    message: String,
}

mod ct;
mod repo;
mod meta;
mod sync;
mod export;
mod import;
mod login;
mod logout;
mod completion;

pub(crate) use self::completion::Completion;
pub(crate) use self::ct::Ct;
pub(crate) use self::export::Export;
pub(crate) use self::import::Import;
pub(crate) use self::login::Login;
pub(crate) use self::logout::Logout;
pub(crate) use self::meta::Meta;
pub(crate) use self::repo::Repo;
pub(crate) use self::sync::Sync_;
