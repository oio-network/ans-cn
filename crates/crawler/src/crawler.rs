use entity::{asn, asn::ISP};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client, Error, IntoUrl,
};
use std::{collections::HashMap, result::Result};

lazy_static! {
    static ref DEFAULT_HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36"
                .parse()
                .unwrap(),
        );
        headers
    };
    static ref CLIETN: Client = Client::builder()
        .default_headers(DEFAULT_HEADERS.clone())
        .build()
        .expect("Failed to create reqwest Client");
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
    static ref ROW_REGEX: Regex = Regex::new(r"<tr>[\s\S]*?</tr>").unwrap();
    static ref ASN_REGEX: Regex =
        Regex::new(r#"<td> ?<a.*?title="(.*?)">AS([0-9]+)</a>\s?</td>"#).unwrap();
    static ref ISP_REGEXES: HashMap<ISP, Regex> = ISP_PATTERNS
        .iter()
        .map(|(isp, pattern)| {
            (
                *isp,
                RegexBuilder::new(pattern)
                    .case_insensitive(true)
                    .build()
                    .unwrap(),
            )
        })
        .collect();
}

pub struct Crawler;

impl Crawler {
    pub async fn asn<U: IntoUrl>(&self, url: U) -> Result<Vec<asn::Model>, Error> {
        let resp = CLIETN.get(url).send().await?;

        Ok(self.extract_asn(resp.text().await?.as_str()).await)
    }

    async fn extract_asn<'a>(&self, text: &'a str) -> Vec<asn::Model> {
        ROW_REGEX
            .find_iter(text)
            .flat_map(|row| ASN_REGEX.captures_iter(row.as_str()))
            .map(|data| {
                let (_, [name, asn]) = data.extract();
                let isp = self.determine_isp(name);
                (
                    asn,
                    asn::Model {
                        number: asn.to_string(),
                        name: name.to_string(),
                        isp,
                        ..Default::default()
                    },
                )
            })
            .collect::<HashMap<&'a str, asn::Model>>()
            .into_values()
            .collect()
    }

    fn determine_isp(&self, name: &str) -> ISP {
        ISP_REGEXES
            .iter()
            .find(|(_, re)| re.is_match(name))
            .map(|(isp, _)| *isp)
            .unwrap_or(ISP::OTHER)
    }
}
