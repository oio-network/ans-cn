use crate::asn::crawler::Crawler;
use crate::asn::model::ASN;
use futures::future::try_join;
use std::collections::HashSet;
use worker::{Env, Result};

pub struct ASNsService {
    env: Env,
    crawler: Crawler,
}

impl ASNsService {
    const DB_NAMESPACE: &'static str = "DB";
    const KV_NAMESPACE: &'static str = "kv";

    pub fn new(env: Env) -> Self {
        ASNsService {
            env,
            crawler: Crawler::new(),
        }
    }

    pub async fn query_all_asn(&self) -> Result<Vec<ASN>> {
        let store = self.env.kv(Self::KV_NAMESPACE)?;
        match store.get("whole").json().await? {
            Some(asn_list) => Ok(asn_list),
            None => {
                let query = "SELECT * FROM ASNs ORDER BY number ASC;";
                let d1 = self.env.d1(Self::DB_NAMESPACE)?;

                match d1.prepare(query).all().await {
                    Ok(result) => {
                        let asn_list = result.results()?;
                        store
                            .put("whole", &asn_list)?
                            .expiration_ttl(600)
                            .execute()
                            .await?;
                        Ok(asn_list)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    pub async fn batch_create_asn(&self, asns: Vec<ASN>) -> Result<()> {
        let query = "INSERT INTO ASNs (number, name) VALUES (?, ?);";
        let d1 = self.env.d1(Self::DB_NAMESPACE)?;

        let statements = asns
            .into_iter()
            .map(|asn| {
                d1.prepare(query)
                    .bind(&[asn.number.into(), asn.name.into()])
                    .unwrap()
            })
            .collect();

        match d1.batch(statements).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_all_asn(&self) -> Result<()> {
        let query = "DELETE FROM ASNs;";
        let d1 = self.env.d1(Self::DB_NAMESPACE)?;

        match d1.prepare(query).run().await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn crawl_asn(&self) -> Result<HashSet<ASN>> {
        let (ipip_net, he_net) = try_join(
            self.crawler.asn("https://whois.ipip.net/iso/CN"),
            self.crawler.asn("https://bgp.he.net/country/CN"),
        )
        .await?;

        Ok(ipip_net.into_iter().chain(he_net.into_iter()).collect())
    }
}
