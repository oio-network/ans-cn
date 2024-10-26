use std::{collections::HashMap, result::Result};

use entity::{asn, asn::ISP};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client, Error, IntoUrl,
};

pub struct CrawlerConfig {
    headers: HeaderMap,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
        Self { headers }
    }
}

lazy_static! {
    static ref ISP_PATTERNS: HashMap<ISP, &'static str> = {
        let mut m = HashMap::new();
        m.insert(ISP::CT, r"CHINA ?TELECOM|CHINANET|CT (?:(?:SHANXI|JIANGXI|DONGGUAN|FOSHAN|ANQING|NEIMENGGU WULANCHABU|LIAONING SHENYANG|CHONGQING|HEFEI NANGANG|GUANGZHOU|HANGZHOU|HUNAN (?:HENGYANG|CHANGSHA)|CENTRALSOUTH CHINA) (?:MAN(?: ?2)?|IDC|IIP)|ESURFINGCLOUD CDN|CNGI)");
        m.insert(
            ISP::CMCC,
            r"China (?:Mobile(?: Communications Corporation)?|TieTong(?: Telecommunications Corporation| SHANGHAI)?)",
        );
        m.insert(ISP::CU, r"UNICOM");
        m.insert(ISP::CERNET, r"CERNET");
        m.insert(ISP::CSTNET, r"CNIC-CAS");
        m.insert(ISP::CAICT, r"Chinese Academy of Telecommunication Research");
        m
    };
}

pub struct Crawler {
    config: CrawlerConfig,
    client: Client,
    row_re: Regex,
    asn_re: Regex,
    isp_regexes: HashMap<ISP, Regex>,
}

impl Crawler {
    pub fn new(config: CrawlerConfig) -> Self {
        let isp_regexes = ISP_PATTERNS
            .iter()
            .map(|(isp, pattern)| {
                (
                    isp.clone(),
                    RegexBuilder::new(pattern)
                        .case_insensitive(true)
                        .build()
                        .unwrap(),
                )
            })
            .collect();

        Crawler {
            config,
            client: Client::new(),
            row_re: Regex::new(r"<tr>[\s\S]*?</tr>").unwrap(),
            asn_re: Regex::new(r#"<td> ?<a.*?title="(.*?)">AS([0-9]+)</a>\s?</td>"#).unwrap(),
            isp_regexes,
        }
    }

    pub async fn asn<U: IntoUrl>(&self, url: U) -> Result<Vec<asn::Model>, Error> {
        let resp = self
            .client
            .get(url)
            .headers(self.config.headers.clone())
            .send()
            .await?;

        Ok(self.extract_asn(resp.text().await?.as_str()).await)
    }

    async fn extract_asn(&self, text: &str) -> Vec<asn::Model> {
        self.row_re
            .find_iter(text)
            .flat_map(|row| self.asn_re.captures_iter(row.as_str()))
            .map(|data| {
                let (_, [name, asn]) = data.extract();
                let isp = self.determine_isp(name);
                (
                    asn.to_string(),
                    asn::Model {
                        number: asn.to_string(),
                        name: name.to_string(),
                        isp,
                        ..Default::default()
                    },
                )
            })
            .collect::<HashMap<String, asn::Model>>()
            .into_values()
            .collect()
    }

    fn determine_isp(&self, name: &str) -> ISP {
        self.isp_regexes
            .iter()
            .find(|(_, re)| re.is_match(name))
            .map(|(isp, _)| *isp)
            .unwrap_or(ISP::OTHER)
    }
}
