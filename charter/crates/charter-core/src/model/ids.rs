use serde::Serialize;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(transparent)]
pub struct AreaId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(transparent)]
pub struct SessionId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(transparent)]
pub struct ResolutionId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(transparent)]
pub struct CandidateId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
#[serde(transparent)]
pub struct ActorId(pub String);
