use super::ast::{Node, PropValue};

/// A Path is a sequence of child indices reaching a specific node.
/// e.g. [0, 2, 1] means "root's 1st child's 3rd child's 2nd child"
pub type Path = Vec<usize>;

#[derive(Debug, Clone, PartialEq)]
pub enum Patch {
    /// Completely replace the node at `path` with a new `node`.
    Replace { path: Path, node: Node },
    
    /// The node type is the same, but its properties changed.
    UpdateProps { path: Path, props: Vec<(String, PropValue)> },
    
    /// Add a new child to the end of the children array of the node at `path`.
    AppendChild { path: Path, node: Node },
    
    /// Remove children from the end of the array to match the new `length`.
    TruncateChildren { path: Path, length: usize },
}

pub fn diff(old: &Node, new: &Node) -> Vec<Patch> {
    let mut patches = Vec::new();
    let mut path = Vec::new();
    diff_recursive(old, new, &mut path, &mut patches);
    patches
}

fn diff_recursive(old: &Node, new: &Node, path: &mut Path, patches: &mut Vec<Patch>) {
    match (old, new) {
        (Node::Element { name: old_name, props: old_props, children: old_children },
         Node::Element { name: new_name, props: new_props, children: new_children }) => {
             
            // 1. If the component type changed completely, replace the whole thing.
            if old_name != new_name {
                patches.push(Patch::Replace { path: path.clone(), node: new.clone() });
                return;
            }

            // 1b. If this is a page element and its identity (title or presentation) changed, replace it!
            if old_name == "page" {
                let get_prop = |props: &[(String, PropValue)], key: &str| -> Option<String> {
                    props.iter().find(|(k, _)| k == key).map(|(_, v)| match v {
                        PropValue::String(s) => s.clone(),
                        _ => format!("{:?}", v),
                    })
                };
                let old_title = get_prop(old_props, "title");
                let new_title = get_prop(new_props, "title");
                let old_pres = get_prop(old_props, "presentation");
                let new_pres = get_prop(new_props, "presentation");
                if old_title != new_title || old_pres != new_pres {
                    patches.push(Patch::Replace { path: path.clone(), node: new.clone() });
                    return;
                }
            }
            
            // 2. Diff Properties
            // A simple equality check is incredibly fast now that we derived PartialEq
            if old_props != new_props {
                patches.push(Patch::UpdateProps { path: path.clone(), props: new_props.clone() });
            }
            
            // 3. Diff Children
            let min_len = std::cmp::min(old_children.len(), new_children.len());
            
            // Diff the overlapping children
            for i in 0..min_len {
                path.push(i);
                diff_recursive(&old_children[i], &new_children[i], path, patches);
                path.pop();
            }
            
            // If the new tree has MORE children, append them
            if new_children.len() > old_children.len() {
                for i in min_len..new_children.len() {
                    patches.push(Patch::AppendChild { path: path.clone(), node: new_children[i].clone() });
                }
            } 
            // If the old tree had MORE children, truncate them
            else if old_children.len() > new_children.len() {
                patches.push(Patch::TruncateChildren { path: path.clone(), length: new_children.len() });
            }
        }
        
        (Node::RawJson(o), Node::RawJson(n)) => {
            if o != n {
                patches.push(Patch::Replace { path: path.clone(), node: new.clone() });
            }
        }
        
        // If node types mismatched (Element vs RawJson), replace the whole thing
        _ => {
            patches.push(Patch::Replace { path: path.clone(), node: new.clone() });
        }
    }
}
