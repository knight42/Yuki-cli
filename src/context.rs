use reqwest::{self, Url};
use reqwest::header::Headers;

pub struct ContextBuilder<'a> {
    pub remote: &'a str,
    pub cli_builder: reqwest::ClientBuilder,
}

impl<'a> ContextBuilder<'a> {
    #[allow(dead_code)]
    pub fn set_remote(&mut self, url: &'a str) -> &mut Self {
        self.remote = url;
        self
    }

    #[allow(dead_code)]
    pub fn set_default_headers(&mut self, headers: Headers) -> &mut Self {
        self.cli_builder.default_headers(headers);
        self
    }

    pub fn build(mut self) -> ::Result<Context> {
        Ok(Context {
            remote: Url::parse(self.remote)?.join("api/v1/")?,
            client: self.cli_builder.build()?,
        })
    }
}

pub struct Context {
    pub remote: Url,
    pub client: reqwest::Client,
}

const DEFAULT_REMOTE: &str = "http://127.0.0.1:9999/";

impl<'a> Context {
    pub fn builder() -> ContextBuilder<'a> {
        ContextBuilder {
            remote: DEFAULT_REMOTE,
            cli_builder: reqwest::ClientBuilder::new(),
        }
    }
}
