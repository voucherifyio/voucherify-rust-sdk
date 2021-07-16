pub mod get;
pub mod list;

use std::collections::BTreeMap;
use serde_json::Value;

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct AsyncActionList {
    pub async_actions: Vec<AsyncAction>
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct AsyncAction {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub async_action_type: Option<String>,
    pub status: Option<String>,
    pub result: Option<BTreeMap<String, Value>>,
    pub created_at: Option<String>,
}

impl AsyncAction {
    pub fn new() -> AsyncAction {
        AsyncAction::default()
    }

    pub fn id(mut self, id: String) -> AsyncAction {
        self.id = Some(id);
        self
    }

    pub fn async_action_type(mut self, async_action_type: String) -> AsyncAction {
        self.async_action_type = Some(async_action_type);
        self
    }

    pub fn status(mut self, status: String) -> AsyncAction {
        self.status = Some(status);
        self
    }

    pub fn result(mut self, result: BTreeMap<String, Value>) -> AsyncAction {
        self.result = Some(result);
        self
    }

    pub fn created_at(mut self, created_at: String) -> AsyncAction {
        self.created_at = Some(created_at);
        self
    }

    pub fn build(self) -> AsyncAction {
        self
    }
}
