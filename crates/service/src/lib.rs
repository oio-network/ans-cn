mod mutation;
mod orm;
mod query;

pub use mutation::*;
pub use query::*;

pub(crate) const DB_NAMESPACE: &'static str = "DB";
pub(crate) const KV_NAMESPACE: &'static str = "kv";

pub(crate) const DB_CHUNK_SIZE: usize = 16;
pub(crate) const KV_EXPIRATION_TTL: u64 = 3600;
