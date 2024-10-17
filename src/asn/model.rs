use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum ISP {
    OTHER,
    CT,
    CMCC,
    CU,
    CERNET,
    CSTNET,
    CAICT,
}

#[derive(Debug, Eq, Serialize, Deserialize)]
pub struct ASN {
    pub number: String,
    pub name: String,
    pub isp: ISP,
}

impl Hash for ASN {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.number, state);
    }
}

impl PartialEq for ASN {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}
