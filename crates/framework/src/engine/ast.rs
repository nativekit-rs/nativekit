use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum PropValue {
    String(String),
    Bool(bool),
    Int(u32),
    ServerFn(String),
    StateRef(String),
    Json(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element {
        name: String,
        props: Vec<(String, PropValue)>,
        children: Vec<Node>,
    },
    RawJson(Value),
    RawBinary(String),
}

impl Node {

    pub fn from_json_value(val: Value) -> Node {
        if let Some(obj) = val.as_object() {
            let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("div").to_string();
            let mut props_vec = Vec::new();
            if let Some(props) = obj.get("props").and_then(|v| v.as_object()) {
                for (k, v) in props {
                    if let Some(s) = v.as_str() {
                        if k == "server_fn" {
                            props_vec.push((k.clone(), PropValue::ServerFn(s.to_string())));
                        } else {
                            props_vec.push((k.clone(), PropValue::String(s.to_string())));
                        }
                    } else if let Some(b) = v.as_bool() {
                        props_vec.push((k.clone(), PropValue::Bool(b)));
                    } else if let Some(n) = v.as_u64() {
                        props_vec.push((k.clone(), PropValue::Int(n as u32)));
                    } else if v.is_object() || v.is_array() {
                        if let Some(sref) = v.get("$type") {
                            if sref == "state_ref" {
                                if let Some(id) = v.get("id").and_then(|i| i.as_str()) {
                                    props_vec.push((k.clone(), PropValue::StateRef(id.to_string())));
                                    continue;
                                }
                            }
                        }
                        props_vec.push((k.clone(), PropValue::Json(v.clone())));
                    }
                }
            }
            let mut children_vec = Vec::new();
            if let Some(arr) = obj.get("children").and_then(|v| v.as_array()) {
                for child in arr {
                    children_vec.push(Node::from_json_value(child.clone()));
                }
            }
            Node::Element {
                name,
                props: props_vec,
                children: children_vec,
            }
        } else {
            Node::Element {
                name: "text".to_string(),
                props: vec![("label".to_string(), PropValue::String(val.to_string()))],
                children: vec![],
            }
        }
    }

    pub fn to_json_value(&self) -> Value {

        match self {
            Node::RawJson(val) => val.clone(),
            Node::RawBinary(s) => serde_json::json!({ "raw_binary": s }),
            Node::Element { name, props, children } => {
                let mut prop_map = serde_json::Map::new();
                for (k, v) in props {
                    match v {
                        PropValue::String(s) => { prop_map.insert(k.clone(), json!(s)); }
                        PropValue::Bool(b) => { prop_map.insert(k.clone(), json!(b)); }
                        PropValue::Int(i) => { prop_map.insert(k.clone(), json!(i)); }
                        PropValue::ServerFn(f) => {
                            prop_map.insert("server_fn".to_string(), json!(f));
                            prop_map.insert(k.clone(), json!(f));
                        }
                        PropValue::StateRef(ref id) => {
                            prop_map.insert(k.clone(), json!({ "$type": "state_ref", "id": id }));
                        }
                        PropValue::Json(val) => {
                            prop_map.insert(k.clone(), val.clone());
                        }
                    }
                }
                let child_arr: Vec<Value> = children.iter().map(|c| c.to_json_value()).collect();
                json!({
                    "name": name,
                    "props": prop_map,
                    "children": child_arr
                })
            }
        }
    }

    pub fn get_string_prop(&self, key: &str) -> Option<String> {
        let props = match self { Node::Element { props, .. } => props, _ => return None };
        for (k, v) in props {
            if k == key {
                if let PropValue::String(s) = v {
                    return Some(s.clone());
                }
            }
        }
        None
    }

    pub fn get_bool_prop(&self, key: &str, default: bool) -> bool {
        let props = match self { Node::Element { props, .. } => props, _ => return default };
        for (k, v) in props {
            if k == key {
                if let PropValue::Bool(b) = v {
                    return *b;
                }
            }
        }
        default
    }

    pub fn get_int_prop(&self, key: &str, default: u32) -> u32 {
        let props = match self { Node::Element { props, .. } => props, _ => return default };
        for (k, v) in props {
            if k == key {
                if let PropValue::Int(i) = v {
                    return *i;
                }
            }
        }
        default
    }

    pub fn get_server_fn_prop(&self) -> Option<String> {
        let props = match self { Node::Element { props, .. } => props, _ => return None };
        for (k, v) in props {
            if k == "server_fn" {
                if let PropValue::ServerFn(f) = v {
                    return Some(f.clone());
                }
            }
        }
        None
    }

    pub fn get_state_ref_prop(&self, key: &str) -> Option<String> {
        let props = match self { Node::Element { props, .. } => props, _ => return None };
        for (k, v) in props {
            if k == key {
                if let PropValue::StateRef(id) = v {
                    return Some(id.clone());
                }
            }
        }
        None
    }
}
