use serde::Serialize;

#[serde(transparent)]
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub struct AreaId(pub String);

#[serde(transparent)]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SessionId(pub String);

#[serde(transparent)]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ResolutionId(pub String);

#[serde(transparent)]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CandidateId(pub String);

#[serde(transparent)]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ActorId(pub String);
