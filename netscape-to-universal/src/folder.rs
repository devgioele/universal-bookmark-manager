use crate::node_ref_ext::*;
use crate::tree_node::TreeNode;
use derive_builder::Builder;
use kuchiki::NodeRef;
use std::fs::File;

#[derive(Clone, Builder, Debug)]
pub struct Folder {
    pub title: String,
    pub children: Vec<TreeNode>,
}

impl Folder {
    pub fn from_node(
        node: &NodeRef,

        mut ancestors: Vec<String>,

        output_file: &mut File,
    ) -> Option<Self> {
        let mut folder = None;

        if node.is_element("DT") {
            let h3 = node.children().find(|n| n.is_element("H3"));

            if let Some(node) = h3 {
                folder = Folder::from_node(&node, ancestors, output_file);
            }
        } else if node.is_element("H3") {
            let mut builder = FolderBuilder::default();

            let title = node.text_contents();

            builder.title(title.clone());

            // Add title to list of ancestors for the children nodes to come

            ancestors.push(format!("{}{}", "#", title.replace(' ', "-")));

            for sibling in node.following_siblings() {
                if sibling.is_element("DL") {
                    let mut children = vec![];

                    for child in sibling.children() {
                        if let Some(node) =
                            TreeNode::from_node(&child, ancestors.clone(), output_file)
                        {
                            children.push(node)
                        }
                    }

                    builder.children(children);
                }
            }

            if let Ok(built) = builder.build() {
                folder = Some(built);
            }
        }

        folder
    }
}
