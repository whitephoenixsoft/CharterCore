use serde::Serialize;

pub type Label = String;
pub type DisplayName = String;
pub type Annotation = String;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize)]
pub struct AreaId(pub Label);

/* 
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SessionId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ResolutionId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CandidateId(pub String);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ActorId(pub String);
*/
