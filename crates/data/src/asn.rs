use ::entity::{asn, asn::Entity as ASN};
use biz::ASNRepo as BizASNRepo;
use pkgs::Error;
use sea_orm::{
    sea_query, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    TransactionTrait,
};
use std::sync::Arc;
use worker::kv::KvStore;

pub struct ASNRepo {
    kv: Arc<KvStore>,
    d1: Arc<DatabaseConnection>,
}

impl ASNRepo {
    pub fn new(kv: Arc<KvStore>, d1: Arc<DatabaseConnection>) -> Self {
        Self { kv, d1 }
    }
}

#[async_trait::async_trait(?Send)]
impl BizASNRepo for ASNRepo {
    async fn query_all(&self) -> Result<Vec<asn::Model>, Error> {
        if let Some(asns) = self.kv.get("whole").json::<Vec<asn::Model>>().await? {
            return Ok(asns);
        }

        let asns = ASN::find().all(self.d1.as_ref()).await?;

        self.kv
            .put("whole", &asns)?
            .expiration_ttl(crate::KV_EXPIRATION_TTL)
            .execute()
            .await?;

        Ok(asns)
    }

    async fn bulk_upsert(&self, asns: Vec<asn::Model>) -> Result<(), Error> {
        let now = chrono::Utc::now().timestamp_millis();
        let inserts: Vec<asn::ActiveModel> = asns
            .into_iter()
            .map(|model| asn::ActiveModel {
                updated_at: Set(now),
                number: Set(model.number),
                name: Set(model.name),
                isp: Set(model.isp),
                ..Default::default()
            })
            .collect();

        let txn = self.d1.begin().await?;
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
                .exec(&txn)
                .await?;
        }
        txn.commit().await?;

        Ok(())
    }

    async fn delete_expired(&self, expired_in: chrono::Duration) -> Result<u64, Error> {
        let now = chrono::Utc::now().timestamp_millis();

        let del_res = ASN::delete_many()
            .filter(asn::Column::UpdatedAt.gte(now - expired_in.num_milliseconds()))
            .exec(self.d1.as_ref())
            .await?;

        Ok(del_res.rows_affected)
    }
}
