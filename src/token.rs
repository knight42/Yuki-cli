use std::io::Read;

use serde_json;
use failure::err_msg;

#[derive(Serialize, Deserialize)]
pub(crate) struct UserCredential<'a> {
    pub host: &'a str,
    pub token: &'a str,
}

pub(crate) fn find_by_host<R: Read>(mut r: R, host: &str) -> ::Result<String> {
    let mut buffer = vec![];
    r.read_to_end(&mut buffer)?;
    let ts: Vec<UserCredential> = serde_json::from_slice(buffer.as_slice())?;
    ts.into_iter()
        .filter(|x| x.host == host)
        .next()
        .map(|x| String::from(x.token))
        .ok_or(err_msg("no such host"))
}
