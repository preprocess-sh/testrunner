use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TestRun {
    pub id: String,
    pub language: String,
    pub payload: HashMap<String, String>,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Event {
    Created { testrun: TestRun },
    Updated { old: TestRun, new: TestRun },
    Deleted { testrun: TestRun },
}

impl Event {
    pub fn id(&self) -> &str {
        match self {
            Event::Created { testrun } => testrun.id.as_str(),
            Event::Updated { new, .. } => new.id.as_str(),
            Event::Deleted { testrun } => testrun.id.as_str(),
        }
    }
}
