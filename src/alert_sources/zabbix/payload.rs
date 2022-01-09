use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub groupid: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Host {
    pub hostid: String,
    pub name: String,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LastEvent {
    pub eventid: String,
    pub acknowledged: String,
    pub value: String,
    pub clock: String,
    pub objectid: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub tag: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActiveTrigger {
    pub triggerid: String,
    pub description: String,
    pub priority: String,
    pub lastchange: String,
    pub value: String,
    pub state: String,
    pub groups: Vec<Group>,
    pub hosts: Vec<Host>,
    pub tags: Vec<Tag>,
    pub lastEvent: LastEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub eventid: String,
    pub objectid: String,
    pub clock: String,
    pub value: String,
    pub acknowledged: String,
    pub name: String,
    pub severity: String,
    pub correlationid: String,
    pub userid: String,
    pub suppressed: String,
    pub tags: Vec<Tag>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
