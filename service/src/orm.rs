use std::{collections::BTreeMap, sync::Arc};
use wasm_bindgen::JsValue;

use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, ProxyDatabaseTrait,
    ProxyExecResult, ProxyRow, RuntimeErr, Schema, Statement, Value, Values,
};
use worker::{Env, Error, Result};

struct D1Proxy {
    env: Arc<Env>,
    binding: String,
}

impl std::fmt::Debug for D1Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("D1Proxy").finish()
    }
}

impl D1Proxy {
    async fn query(env: Arc<Env>, binding: String, statement: Statement) -> Result<Vec<ProxyRow>> {
        let sql = statement.sql.clone();
        let values = match statement.values {
            Some(Values(values)) => values
                .iter()
                .map(|val| match &val {
                    Value::BigInt(Some(val)) => JsValue::from(val.to_string()),
                    Value::BigUnsigned(Some(val)) => JsValue::from(val.to_string()),
                    Value::Int(Some(val)) => JsValue::from(*val),
                    Value::Unsigned(Some(val)) => JsValue::from(*val),
                    Value::SmallInt(Some(val)) => JsValue::from(*val),
                    Value::SmallUnsigned(Some(val)) => JsValue::from(*val),
                    Value::TinyInt(Some(val)) => JsValue::from(*val),
                    Value::TinyUnsigned(Some(val)) => JsValue::from(*val),

                    Value::Float(Some(val)) => JsValue::from_f64(*val as f64),
                    Value::Double(Some(val)) => JsValue::from_f64(*val),

                    Value::Bool(Some(val)) => JsValue::from(*val),
                    Value::Bytes(Some(val)) => JsValue::from(format!(
                        "X'{}'",
                        val.iter()
                            .map(|byte| format!("{:02x}", byte))
                            .collect::<String>()
                    )),
                    Value::Char(Some(val)) => JsValue::from(val.to_string()),
                    Value::Json(Some(val)) => JsValue::from(val.to_string()),
                    Value::String(Some(val)) => JsValue::from(val.to_string()),

                    Value::ChronoDate(Some(val)) => JsValue::from(val.to_string()),
                    Value::ChronoDateTime(Some(val)) => JsValue::from(val.to_string()),
                    Value::ChronoDateTimeLocal(Some(val)) => JsValue::from(val.to_string()),
                    Value::ChronoDateTimeUtc(Some(val)) => JsValue::from(val.to_string()),
                    Value::ChronoDateTimeWithTimeZone(Some(val)) => JsValue::from(val.to_string()),

                    _ => JsValue::NULL,
                })
                .collect(),
            None => Vec::new(),
        };

        let ret = env
            .d1(binding.as_str())?
            .prepare(sql)
            .bind(&values)?
            .all()
            .await?;

        let ret = ret.results::<serde_json::Value>()?;
        let ret = ret
            .iter()
            .map(|row| {
                let mut values = BTreeMap::new();
                for (key, value) in row.as_object().unwrap() {
                    values.insert(
                        key.clone(),
                        match &value {
                            serde_json::Value::Bool(val) => Value::Bool(Some(*val)),
                            serde_json::Value::Number(val) => {
                                if val.is_i64() {
                                    Value::BigInt(Some(val.as_i64().unwrap()))
                                } else if val.is_u64() {
                                    Value::BigUnsigned(Some(val.as_u64().unwrap()))
                                } else {
                                    Value::Double(Some(val.as_f64().unwrap()))
                                }
                            }
                            serde_json::Value::String(val) => {
                                Value::String(Some(Box::new(val.clone())))
                            }
                            _ => unreachable!("Unsupported JSON value"),
                        },
                    );
                }
                ProxyRow { values }
            })
            .collect();

        Ok(ret)
    }

    async fn execute(
        env: Arc<Env>,
        binding: String,
        statement: Statement,
    ) -> Result<ProxyExecResult> {
        let sql = statement.sql.clone();
        let values = match statement.values {
            Some(Values(values)) => values
                .iter()
                .map(|val| match &val {
                    Value::BigInt(Some(val)) => JsValue::from(val.to_string()),
                    Value::BigUnsigned(Some(val)) => JsValue::from(val.to_string()),
                    Value::Int(Some(val)) => JsValue::from(*val),
                    Value::Unsigned(Some(val)) => JsValue::from(*val),
                    Value::SmallInt(Some(val)) => JsValue::from(*val),
                    Value::SmallUnsigned(Some(val)) => JsValue::from(*val),
                    Value::TinyInt(Some(val)) => JsValue::from(*val),
                    Value::TinyUnsigned(Some(val)) => JsValue::from(*val),

                    Value::Float(Some(val)) => JsValue::from_f64(*val as f64),
                    Value::Double(Some(val)) => JsValue::from_f64(*val),

                    Value::Bool(Some(val)) => JsValue::from(*val),
                    Value::Bytes(Some(val)) => JsValue::from(format!(
                        "X'{}'",
                        val.iter()
                            .map(|byte| format!("{:02x}", byte))
                            .collect::<String>()
                    )),
                    Value::Char(Some(val)) => JsValue::from(val.to_string()),
                    Value::Json(Some(val)) => JsValue::from(val.to_string()),
                    Value::String(Some(val)) => JsValue::from(val.to_string()),

                    Value::ChronoDate(Some(val)) => JsValue::from(val.to_string()),
                    Value::ChronoDateTime(Some(val)) => JsValue::from(val.to_string()),
                    Value::ChronoDateTimeLocal(Some(val)) => JsValue::from(val.to_string()),
                    Value::ChronoDateTimeUtc(Some(val)) => JsValue::from(val.to_string()),
                    Value::ChronoDateTimeWithTimeZone(Some(val)) => JsValue::from(val.to_string()),

                    _ => JsValue::NULL,
                })
                .collect(),
            None => Vec::new(),
        };

        let ret = env
            .d1(binding.as_str())?
            .prepare(sql)
            .bind(&values)?
            .run()
            .await?
            .meta()?;

        let last_insert_id = ret
            .as_ref()
            .map(|meta| meta.last_row_id.unwrap_or(0))
            .unwrap_or(0) as u64;
        let rows_affected = ret
            .as_ref()
            .map(|meta| meta.rows_written.unwrap_or(0))
            .unwrap_or(0) as u64;

        Ok(ProxyExecResult {
            last_insert_id,
            rows_affected,
        })
    }
}

#[async_trait::async_trait]
impl ProxyDatabaseTrait for D1Proxy {
    async fn query(&self, statement: Statement) -> std::result::Result<Vec<ProxyRow>, DbErr> {
        let (env, binding) = (self.env.clone(), self.binding.clone());
        let (tx, rx) = oneshot::channel();
        wasm_bindgen_futures::spawn_local(async move {
            let ret = Self::query(env, binding, statement).await;
            tx.send(ret).unwrap();
        });

        let ret = rx.await.unwrap();
        ret.map_err(|err| DbErr::Conn(RuntimeErr::Internal(err.to_string())))
    }

    async fn execute(&self, statement: Statement) -> std::result::Result<ProxyExecResult, DbErr> {
        let (env, binding) = (self.env.clone(), self.binding.clone());
        let (tx, rx) = oneshot::channel();
        wasm_bindgen_futures::spawn_local(async move {
            let ret = Self::execute(env, binding, statement).await;
            tx.send(ret).unwrap();
        });

        let ret = rx.await.unwrap();
        ret.map_err(|err| DbErr::Conn(RuntimeErr::Internal(err.to_string())))
    }
}

pub async fn d1(env: Arc<Env>, binding: String) -> Result<DatabaseConnection> {
    let db = Database::connect_proxy(
        DbBackend::Sqlite,
        Arc::new(Box::new(D1Proxy { env, binding })),
    )
    .await
    .map_err(|e| Error::RustError(format!("Failed to connect to database: {:?}", e)))?;

    let builder = db.get_database_backend();

    db.execute(
        builder.build(
            Schema::new(builder)
                .create_table_from_entity(entity::asn::Entity)
                .if_not_exists(),
        ),
    )
    .await
    .map_err(|e| Error::RustError(format!("Failed to execute in database: {:?}", e)))?;

    Ok(db)
}
