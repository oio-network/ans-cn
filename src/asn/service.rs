use std::collections::HashSet;
use std::error::Error;
use futures::future::try_join;
use crate::asn::crawler::Crawler;
use crate::asn::model::ASN;


pub struct ASNsService {
    crawler: Crawler,
}

impl ASNsService {
    pub fn new() -> Self {
        ASNsService {
            crawler: Crawler::new()
        }
    }

    pub async fn get_asn(&self) -> Result<HashSet<ASN>, Box<dyn Error>> {
        let (ipip_net, he_net) = try_join(
            self.crawler.asn("https://whois.ipip.net/iso/CN"),
            self.crawler.asn("https://bgp.he.net/country/CN"),
        ).await?;

        Ok(ipip_net.into_iter().chain(he_net.into_iter()).collect())
    }
}