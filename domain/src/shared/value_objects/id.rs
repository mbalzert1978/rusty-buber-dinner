use std::str::FromStr;
use std::sync::Arc;

use crate::prelude::*;
use crate::DomainError;

use super::traits::GetValues;

pub(crate) struct UuidId(uuid::Uuid);

impl GetValues for UuidId {
    type Values = uuid::Uuid;

    fn get_values(&self) -> Arc<Self::Values> {
        Arc::new(self.0)
    }
}

impl From<uuid::Uuid> for UuidId {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for UuidId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            uuid::Uuid::try_parse(s).map_err(|err| DomainError::InvalidUuid(err.to_string()))?,
        ))
    }
}
