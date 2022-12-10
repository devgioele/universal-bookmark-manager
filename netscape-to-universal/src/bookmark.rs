use crate::node_ref_ext::*;
use derive_builder::Builder;
use kuchiki::NodeRef;
use std::{fs::File, io::Write};

#[derive(Clone, Builder, Debug)]
pub struct Bookmark {
    pub href: String,
}

impl Bookmark {
    pub fn from_node(
        node: &NodeRef,
        ancestors: Vec<String>,
        output_file: &mut File,
    ) -> Option<Self> {
        let mut bookmark = None;
        let mut builder = BookmarkBuilder::default();

        if node.is_element("DT") {
            let a = node.children().find(|n| n.is_element("A"));

            if let Some(node) = a {
                bookmark = Bookmark::from_node(&node, ancestors, output_file);
            }
        } else if node.is_element("A") {
            if let Some(attribute) = node.select_attribute("HREF") {
                builder.href(attribute.value);
            }

            if let Ok(built) = builder.build() {
                // Write to file
                if output_file
                    .write(format!("{} {}\n", built.href, ancestors.join(" ")).as_bytes())
                    .is_err()
                {
                    panic!("Cannot write to file: {:?}", output_file);
                }
                bookmark = Some(built);
            }
        }

        bookmark
    }
}
