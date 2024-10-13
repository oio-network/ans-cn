use regex::Regex;
use reqwest::{Client, Error, IntoUrl};

pub struct Crawler {
    client: Client,
    tab_re: Regex,
    asn_re: Regex,
}

impl Crawler {
    pub fn new() -> Self {
        Crawler {
            client: Client::new(),
            tab_re: Regex::new(r#"<tr>(.*?)</tr>"#).unwrap(),
            asn_re: Regex::new(r#"<td> ?<a.*?title="(.*?)AS([0-9]+)</a>\s?</td>"#).unwrap(),
        }
    }

    pub async fn asn<U: IntoUrl>(&self, client: &Client, url: U) -> Result<String, Error> {
        match client.get(url).send().await {
            Ok(resp) => self.extract_asn(resp.text().await?.as_str()),
            Err(err) => err,
        }
    }

    async fn extract_asn(&self, text: &str) {
        let lines: Vec<&str> = self.tab_re.find_iter(text).collect();
        for line in lines {
            let a: Vec<&str, &str> = self.asn_re.find_iter(line).collect();
        }
    }
}

pub async fn get_asn<U: IntoUrl>(client: &Client, url: U) -> Result<String, Error> {
    let resp = client.get(url).send().await?;
    Ok(resp.text().await?)
}
