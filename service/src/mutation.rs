use std::sync::Arc;

use ::entity::{asn, asn::Entity as ASN};
use sea_orm::{ActiveValue::Set, *};
use worker::{Env, Error, Result};

use crate::orm;

pub struct Mutation;

impl Mutation {
    pub async fn bulk_upsert(env: Arc<Env>, asns: Vec<asn::Model>) -> Result<()> {
        let now = chrono::Utc::now().timestamp_millis();
        let inserts: Vec<asn::ActiveModel> = asns
            .iter()
            .map(|model| asn::ActiveModel {
                updated_at: Set(now),
                number: Set(model.number.clone()),
                name: Set(model.name.clone()),
                isp: Set(model.isp.clone()),
                ..Default::default()
            })
            .collect();

        let db = orm::d1(env, crate::DB_NAMESPACE.to_string()).await?;
        for chunk in inserts.chunks(crate::DB_CHUNK_SIZE) {
            ASN::insert_many(chunk.to_vec())
                .on_conflict(
                    sea_query::OnConflict::column(asn::Column::Number)
                        .update_columns([
                            asn::Column::UpdatedAt,
                            asn::Column::Name,
                            asn::Column::Isp,
                        ])
                        .to_owned(),
                )
                .exec(&db)
                .await
                .map_err(|e| Error::RustError(format!("failed to bulk upsert: {:?}", e)))?;
        }

        Ok(())
    }

    pub async fn delete_expired(env: Arc<Env>, expired_in: chrono::Duration) -> Result<()> {
        let now = chrono::Utc::now().timestamp_millis();
        let db = orm::d1(env, crate::DB_NAMESPACE.to_string()).await?;

        match ASN::delete_many()
            .filter(asn::Column::UpdatedAt.gte(now - expired_in.num_milliseconds()))
            .exec(&db)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::RustError(format!(
                "failed to delete expired: {:?}",
                e
            ))),
        }
    }
}
