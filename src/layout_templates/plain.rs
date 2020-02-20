use libwispha::core::*;

use crate::layouter::*;
use super::resolve_handler;

use std::error;

pub struct PlainLayout { }

impl PlainLayout {
    pub fn new() -> PlainLayout {
        PlainLayout { }
    }

    // if `depth` < `max`, return `Some`, else return `None`
    fn layout_helper(&self,
                     tree: &Tree,
                     node_path: &NodePath,
                     depth: usize,
                     max: usize,
                     keys: &Vec<String>,
                     hide_key: bool) -> Option<String> {
        if depth <= max {
            let mut line = String::new();
            // Can safely unwrap because of the effect of `resolve_node`
            let node = tree.get_node(node_path).unwrap();
            let node = node.borrow();
            let direct_node = node.get_direct().unwrap();

            line += &vec!["\t"; depth].concat();

            line += &direct_node.node_properties.name;

            if keys.len() == 1 {
                let key = &keys[0];
                if let Some(property) = direct_node.properties.get(key) {
                    line += "\t\t";
                    if !hide_key {
                        line += key;
                        line += ": ";
                    }
                    line += property;
                }
            } else if keys.len() > 1 {
                for key in keys {
                    if let Some(property) = direct_node.properties.get(key) {
                        line += "\t\t";
                        line += key;
                        line += ": ";
                        line += property;
                    }
                }
            }

            let mut sub_lines = direct_node.children.iter().filter_map(|child_path| {
                self.layout_helper(tree, child_path, depth + 1, max, keys, hide_key)
            }).collect::<Vec<String>>();

            sub_lines.insert(0, line);

            Some(sub_lines.join("\n"))
        } else {
            None
        }
    }
}

impl Layout for PlainLayout {
    fn info(&self) -> LayoutInfo {
        LayoutInfo {
            name: "plain".to_string(),
            version: "1.0".to_string()
        }
    }

    fn layout(&self,
              tree: &Tree,
              node_path: &NodePath,
              depth: usize,
              keys: &Vec<String>,
              hide_key: bool) -> Result<String, Box<dyn error::Error>> {
        tree.resolve_node(node_path, &resolve_handler, &*crate::PRESERVED_KEYS)?;
        tree.resolve_in_depth(node_path, depth, &resolve_handler, &*crate::PRESERVED_KEYS)?;
        Ok(self.layout_helper(tree,
                              node_path,
                              0,
                              depth,
                              keys,
                              hide_key).unwrap())
    }
}