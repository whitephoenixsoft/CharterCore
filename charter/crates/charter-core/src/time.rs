use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use derive_more::Display;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Display )]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// EXPORT: Turns the timestamp into a string 
    pub fn to_key_string(&self) -> String {
        self.0.format("%Y-%m-%dT%H:%M%:%SZ").to_string()
    }
}

/// IMPORT: Allows "mystring".parse::<Timestamp>()
impl FromStr for Timestamp {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dt = DateTime::parse_from_rfc3339(s)?;
        Ok(Self(dt.with_timezone(&Utc)))
    }
}

impl TryFrom<&str> for Timestamp {
    type Error = chrono::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse() //uses FromStr
    }
}

