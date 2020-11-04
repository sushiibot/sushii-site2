use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Permission {
    pub name: String,
    pub color: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Command {
    pub name: String,
    pub aliases: Option<Vec<String>>,
    pub description: String,
    pub required_permissions: Option<Vec<Permission>>,
    pub usage: Option<String>,
    pub examples: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommandList {
    #[serde(flatten)]
    pub groups: HashMap<String, Vec<Command>>,
}
