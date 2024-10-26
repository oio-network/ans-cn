use std::hash::{Hash, Hasher};

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
    DeriveActiveEnum,
    Deserialize,
    Serialize,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    enum_name = "isp"
)]
pub enum ISP {
    #[default]
    #[sea_orm(string_value = "OTHER")]
    OTHER,
    #[sea_orm(string_value = "CT")]
    CT,
    #[sea_orm(string_value = "CMCC")]
    CMCC,
    #[sea_orm(string_value = "CU")]
    CU,
    #[sea_orm(string_value = "CERNET")]
    CERNET,
    #[sea_orm(string_value = "CSTNET")]
    CSTNET,
    #[sea_orm(string_value = "CAICT")]
    CAICT,
}

#[derive(Clone, Debug, Default, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "asns")]
pub struct Model {
    pub updated_at: i64,

    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique, indexed)]
    pub number: String,
    pub name: String,
    pub isp: ISP,
}

impl Hash for Model {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.number, state);
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
