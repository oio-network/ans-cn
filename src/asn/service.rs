use std::collections::HashSet;
use std::error::Error;
use futures::future::try_join;
use worker::{console_log, Env};
use crate::asn::crawler::Crawler;
use crate::asn::model::ASN;


pub struct ASNsService {
    env: Env,
    crawler: Crawler,
}

impl ASNsService {
    pub fn new(env: Env) -> Self {
        ASNsService {
            env,
            crawler: Crawler::new(),
        }
    }

    pub async fn query_all_asn(&self) -> Vec<ASN> {
        let query = "SELECT * FROM ASNs ORDER BY number ASC;";
        let d1 = self.env.d1("DB").unwrap();

        match d1.prepare(query)
            .all()
            .await {
            Ok(rows) => {
                rows.results().unwrap()
            }
            Err(e) => {
                console_log!("failed to query all ASN: {:?}", e);
                vec![]
            }
        }
    }

    pub async fn batch_create_asn(&self, asns: Vec<ASN>) {
        let query = "INSERT INTO ASNs (number, name) VALUES (?, ?);";
        let d1 = self.env.d1("DB").unwrap();

        let statements = asns.into_iter().map(|asn| {
            d1.prepare(query).bind(&[asn.number.into(), asn.name.into()]).unwrap()
        }).collect();

        match d1.batch(statements).await {
            Ok(_) => (),
            Err(e) => console_log!("failed to batch create ASN: {:?}", e),
        }
    }

    pub async fn delete_all_asn(&self) {
        let query = "DELETE FROM ASNs;";
        let d1 = self.env.d1("DB").unwrap();

        d1.prepare(query).run().await.expect("failed to delete all ASN");
    }

    pub async fn crawl_asn(&self) -> Result<HashSet<ASN>, Box<dyn Error>> {
        let (ipip_net, he_net) = try_join(
            self.crawler.asn("https://whois.ipip.net/iso/CN"),
            self.crawler.asn("https://bgp.he.net/country/CN"),
        ).await?;

        Ok(ipip_net.into_iter().chain(he_net.into_iter()).collect())
    }
}