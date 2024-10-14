use std::collections::HashSet;
use std::error::Error;
use regex::Regex;
use reqwest::{Client, IntoUrl, header::USER_AGENT};
use crate::asn::model::ASN;

pub struct Crawler {
    client: Client,
    row_re: Regex,
    asn_re: Regex,
}

impl Crawler {
    pub fn new() -> Self {
        Crawler {
            client: Client::new(),
            row_re: Regex::new(r"<tr>[\s\S]*?</tr>").unwrap(),
            asn_re: Regex::new(r#"<td> ?<a.*?title="(.*?)">AS([0-9]+)</a>\s?</td>"#).unwrap(),
        }
    }

    pub async fn asn<U: IntoUrl>(&self, url: U) -> Result<HashSet<ASN>, Box<dyn Error>> {
        match self.client
            .get(url)
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
            .send()
            .await {
            Ok(resp) => Ok(self.extract_asn(resp.text().await?.as_str()).await),
            Err(e) => Err(e.into()),
        }
    }

    async fn extract_asn(&self, text: &str) -> HashSet<ASN> {
        self.row_re.find_iter(text)
            .flat_map(|row| {
                self.asn_re.captures_iter(row.as_str())
            })
            .map(|data| {
                let (_, [name, asn]) = data.extract();
                ASN {
                    number: asn.to_string(),
                    name: name.to_string(),
                }
            })
            .collect()
    }
}
