use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Serialize, Deserialize)]
pub struct ASN {
    pub number: String,
    pub name: String,
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
