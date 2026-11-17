use super::ast::{Node, PropValue};
use super::diff::Patch;

pub fn encode_string(s: &str, buf: &mut Vec<u8>) {
    let bytes = s.as_bytes();
    buf.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
    buf.extend_from_slice(bytes);
}

pub fn encode_node(node: &Node, buf: &mut Vec<u8>) {
    match node {
        Node::Element { name, props, children } => {
            buf.push(1); // Type 1 = Element
            encode_string(name, buf);
            
            buf.extend_from_slice(&(props.len() as u16).to_le_bytes());
            for (k, v) in props {
                encode_string(k, buf);
                match v {
                    PropValue::String(s) => {
                        buf.push(0);
                        encode_string(s, buf);
                    }
                    PropValue::Bool(b) => {
                        buf.push(1);
                        buf.push(if *b { 1 } else { 0 });
                    }
                    PropValue::Int(i) => {
                        buf.push(2);
                        buf.extend_from_slice(&i.to_le_bytes());
                    }
                    PropValue::ServerFn(s) => {
                        buf.push(3);
                        encode_string(s, buf);
                    }
                    PropValue::StateRef(s) => {
                        buf.push(4);
                        encode_string(s, buf);
                    }
                    PropValue::Json(j) => {
                        buf.push(5);
                        let s = j.to_string();
                        let bytes = s.as_bytes();
                        buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                        buf.extend_from_slice(bytes);
                    }
                }
            }
            
            buf.extend_from_slice(&(children.len() as u16).to_le_bytes());
            for child in children {
                encode_node(child, buf);
            }
        }
        Node::RawJson(val) => {
            buf.push(2); // Type 2 = RawJson
            let s = val.to_string();
            let bytes = s.as_bytes();
            buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            buf.extend_from_slice(bytes);
        }

        Node::RawBinary(b64) => {
            use base64::{Engine as _, engine::general_purpose};
            if let Ok(bytes) = general_purpose::STANDARD.decode(b64) {
                if !bytes.is_empty() { buf.extend_from_slice(&bytes[1..]); }
            } else {
                buf.push(1); // Type 1 = Element
                crate::engine::binary::encode_string("text", buf);
                buf.extend_from_slice(&(1u16).to_le_bytes()); // 1 prop
                crate::engine::binary::encode_string("label", buf);
                buf.push(0); // String prop
                crate::engine::binary::encode_string("Error: OTA payload was not valid Base64 binary. Please update your server.", buf);
                buf.extend_from_slice(&(0u16).to_le_bytes()); // 0 children
            }
        }
    }
}


pub fn encode_patches(patches: &[Patch], buf: &mut Vec<u8>) {
    buf.extend_from_slice(&(patches.len() as u16).to_le_bytes());
    for patch in patches {
        match patch {
            Patch::Replace { path, node } => {
                buf.push(0); // Replace
                encode_path(path, buf);
                encode_node(node, buf);
            }
            Patch::UpdateProps { path, props } => {
                buf.push(1); // UpdateProps
                encode_path(path, buf);
                buf.extend_from_slice(&(props.len() as u16).to_le_bytes());
                for (k, v) in props {
                    encode_string(k, buf);
                    match v {
                        PropValue::String(s) => { buf.push(0); encode_string(s, buf); }
                        PropValue::Bool(b) => { buf.push(1); buf.push(if *b { 1 } else { 0 }); }
                        PropValue::Int(i) => { buf.push(2); buf.extend_from_slice(&i.to_le_bytes()); }
                        PropValue::ServerFn(s) => { buf.push(3); encode_string(s, buf); }
                        PropValue::StateRef(s) => { buf.push(4); encode_string(s, buf); }
                        PropValue::Json(j) => {
                            buf.push(5);
                            let s = j.to_string();
                            let bytes = s.as_bytes();
                            buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                            buf.extend_from_slice(bytes);
                        }
                    }
                }
            }
            Patch::AppendChild { path, node } => {
                buf.push(2); // AppendChild
                encode_path(path, buf);
                encode_node(node, buf);
            }
            Patch::TruncateChildren { path, length } => {
                buf.push(3); // TruncateChildren
                encode_path(path, buf);
                buf.extend_from_slice(&(*length as u16).to_le_bytes());
            }
        }
    }
}

fn encode_path(path: &[usize], buf: &mut Vec<u8>) {
    buf.extend_from_slice(&(path.len() as u16).to_le_bytes());
    for &idx in path {
        buf.extend_from_slice(&(idx as u16).to_le_bytes());
    }
}

use base64::Engine;
pub fn node_to_base64(node: &Node) -> String {
    let mut buf = Vec::new();
    buf.push(0); // Header: 0 = Full Tree
    encode_node(node, &mut buf);
    base64::engine::general_purpose::STANDARD.encode(&buf)
}

pub fn patches_to_base64(patches: &[Patch]) -> String {
    let mut buf = Vec::new();
    buf.push(1); // Header: 1 = Patches
    encode_patches(patches, &mut buf);
    base64::engine::general_purpose::STANDARD.encode(&buf)
}
