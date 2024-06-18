use std::str::FromStr;

use crate::prelude::*;
use crate::DomainError;

pub(crate) struct UuidId(uuid::Uuid);

impl From<uuid::Uuid> for UuidId {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for UuidId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            uuid::Uuid::try_parse(s).map_err(|e| DomainError::InvalidUuid(e.to_string()))?,
        ))
    }
}
