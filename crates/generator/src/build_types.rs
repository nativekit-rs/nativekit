use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct PrimitiveDef {
    pub name: String,
    pub rust: RustDef,
}

#[derive(Debug, Deserialize)]
pub struct RustDef {
    #[serde(default)]
    pub constructor: HashMap<String, String>,
    #[serde(default)]
    pub builder: HashMap<String, String>,
    #[serde(default)]
    pub accepts_children: bool,
}
