use crate::orm;
use ::entity::{asn, asn::Entity as ASN};
use biz::asn::ASNRepo as BizASNRepo;
use pkgs::Error;
use std::sync::Arc;
use sea_orm::EntityTrait;
use worker::Env;

pub struct ASNRepo;

#[async_trait::async_trait]
impl BizASNRepo for ASNRepo {
    async fn query_all(&self, env: Arc<Env>) -> Result<Vec<asn::Model>, Error> {
        let store = env.kv(crate::KV_NAMESPACE)?;
        match store.get("whole").json().await? {
            Some(asns) => Ok(asns),
            None => {
                let db = orm::d1(env, crate::DB_NAMESPACE.to_string()).await?;
                match ASN::find().all(&db).await {
                    Ok(asns) => {
                        store
                            .put("whole", &asns)?
                            .expiration_ttl(crate::KV_EXPIRATION_TTL)
                            .execute()
                            .await?;
                        Ok(asns)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}
