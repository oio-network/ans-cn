use std::sync::Arc;

use ::entity::{asn, asn::Entity as ASN};
use sea_orm::*;
use worker::{Env, Error, Result};

use crate::orm;

pub struct Query;

impl Query {
    pub async fn query_all(env: Arc<Env>) -> Result<Vec<asn::Model>> {
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
                    Err(e) => Err(Error::RustError(format!("failed to query all: {:?}", e))),
                }
            }
        }
    }
}
