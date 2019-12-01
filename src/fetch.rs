use reqwest;

pub trait Fetcher {
    fn get_text(&self, url: &str) -> Result<String, reqwest::Error>;
}

pub struct UrlFetcher {
}

impl Fetcher for UrlFetcher {
    fn get_text(&self, url: &str) -> Result<String, reqwest::Error> {
        let body = reqwest::get(url)?.text()?;

        Ok(body)
    }
}
