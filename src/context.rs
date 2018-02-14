use reqwest::{Client, ClientBuilder, IntoUrl, RequestBuilder, Url};
use reqwest::header::Headers;
use std::path::{Path, PathBuf};

pub struct ContextBuilder<'a> {
    remote: &'a str,
    homedir: &'a Path,
    headers: Headers,
    cli_builder: ClientBuilder,
}

impl<'a> ContextBuilder<'a> {
    pub fn set_remote(&mut self, url: &'a str) -> &mut Self {
        self.remote = url;
        self
    }

    pub fn set_homedir(&mut self, homedir: &'a Path) -> &mut Self {
        self.homedir = homedir;
        self
    }

    pub fn set_headers(&mut self, headers: Headers) -> &mut Self {
        self.headers = headers;
        self
    }

    pub fn build(mut self) -> ::Result<Context> {
        Ok(Context {
            remote: Url::parse(self.remote)?.join("api/v1/")?,
            client: self.cli_builder.build()?,
            headers: self.headers,
            homedir: self.homedir.to_path_buf(),
        })
    }
}

impl<'a> Default for ContextBuilder<'a> {
    fn default() -> Self {
        ContextBuilder {
            remote: DEFAULT_REMOTE,
            homedir: Path::new("/"),
            headers: Headers::default(),
            cli_builder: ClientBuilder::new(),
        }
    }
}

pub struct Context {
    pub remote: Url,
    pub homedir: PathBuf,
    client: Client,
    headers: Headers,
}

pub const DEFAULT_REMOTE: &str = "http://127.0.0.1:9999/";

macro_rules! delegate_methods {
    ($method:ident) => (
    pub fn $method<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        let mut req = self.client.$method(url);
        if self.headers.len() != 0 {
            req.headers(self.headers.clone());
        }
        req
    })
}

impl<'a> Context {
    pub fn builder() -> ContextBuilder<'a> {
        ContextBuilder::default()
    }

    delegate_methods!(get);
    delegate_methods!(post);
    delegate_methods!(put);
    delegate_methods!(delete);
}
