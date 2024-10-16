use crate::asn::model::ASN;
use regex::Regex;
use reqwest::{header::USER_AGENT, Client, IntoUrl};
use std::collections::HashSet;
use worker::{Error, Result};

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

    pub async fn asn<U: IntoUrl>(&self, url: U) -> Result<HashSet<ASN>> {
        let resp = match self.client.get(url)
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
            .send()
            .await {
            Ok(resp) => resp,
            Err(e) => return Err(Error::RustError(format!("failed to fetch: {:?}", e))),
        };

        let text = match resp.text().await {
            Ok(text) => text,
            Err(e) => {
                return Err(Error::RustError(format!(
                    "failed to read response: {:?}",
                    e
                )))
            }
        };

        Ok(self.extract_asn(text.as_str()).await)
    }

    async fn extract_asn(&self, text: &str) -> HashSet<ASN> {
        self.row_re
            .find_iter(text)
            .flat_map(|row| self.asn_re.captures_iter(row.as_str()))
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
