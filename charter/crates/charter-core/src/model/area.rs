use super::ids::{AreaId, ResolutionId};
use crate::time::Timestamp;
use derive_more::{Display, From, FromStr};
use serde::{Serialize,Deserialize};

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, From, FromStr, Display)]
#[serde(transparent)]
pub struct AreaLabel(String);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, From, FromStr, Display)]
#[serde(transparent)]
pub struct AreaName(String);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, From, FromStr, Display)]
#[serde(transparent)]
pub struct Annotation(String);


/// Engine-owned Area record.
/// Areas are hard governance boundaries.
pub struct AreaRuntime {
    /// Unique hash that never changes
    pub area_id: AreaId,

    /// Short label for reference; mutabke
    pub label: AreaLabel,

    /// Area full name; mutable
    pub name: AreaName,

    /// Rationalle around the name (user memory aid); mutable
    pub annotation: Option<Annotation>,

    /// Creation metadata (audit only)
    pub created_at: Timestamp,

    /// Active authority resolution. (required for initialization); mutable
    pub active_authority: Option<ResolutionId>,

    /// Active Scope resolution. (required for initialization); mutable
    pub active_scope: Option<ResolutionId>,
}

impl AreaRuntime {
    pub fn new(label: AreaLabel, name: AreaName, annotation: Option<Annotation>) -> Self {
       Self {
           area_id: AreaId::new(),
           label: label.clone(),
           name: name.clone(),
           annotation: annotation.clone(),
           created_at: Timestamp::now(),
           active_authority: Option::None,
           active_scope: Option::None,
       }
    }
}

