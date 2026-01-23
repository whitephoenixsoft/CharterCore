use uuid::Uuid;
use derive_more::{Display, From, FromStr};
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, Serialize)]
#[serde(transparent)]
pub struct AreaId(pub Uuid);

impl AreaId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl std::str::FromStr for AreaId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}


#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, From, FromStr, Display)]
#[serde(transparent)]
pub struct SessionId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, From, FromStr, Display)]
#[serde(transparent)]
pub struct ResolutionId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, From, FromStr, Display)]
#[serde(transparent)]
pub struct CandidateId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, From, FromStr, Display)]
#[serde(transparent)]
pub struct ActorId(pub String);
