use crate::folder::Folder;
use crate::node_ref_ext::*;
use crate::tree_node::TreeNode;
use kuchiki::NodeRef;
use std::fs::File;

#[derive(Clone, Debug)]
pub struct Tree {
    root: TreeNode,
}

impl Tree {
    pub fn from_node(node: &NodeRef, output_file: &mut File) -> Self {
        let mut title = String::new();
        let mut children = vec![];
        let mut head = None;
        let mut body = None;

        let html = node.children().find(|n| n.is_element("HTML"));

        if let Some(root) = html {
            for child in root.children() {
                if child.is_element("HEAD") {
                    head = Some(child);
                } else if child.is_element("BODY") {
                    body = Some(child);
                }
            }
        }

        if let Some(root) = head {
            for child in root.children() {
                if child.is_element("TITLE") {
                    title = child.text_contents();
                }
            }
        }

        if let Some(root) = body {
            for child in root.children() {
                if child.is_element("DL") {
                    for i in child.children() {
                        if let Some(node) = TreeNode::from_node(&i, vec![], output_file) {
                            children.push(node);
                        }
                    }
                }
            }
        }

        Tree {
            root: TreeNode::FolderNode(Folder { title, children }),
        }
    }
}
