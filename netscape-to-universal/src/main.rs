use crate::node_ref_ext::*;
use kuchiki::{parse_html, traits::TendrilSink, NodeRef};
use std::{
    env,
    fs::File,
    io::{Error, ErrorKind},
    path::Path,
};

mod node_ref_ext;

fn main() -> Result<(), Error> {
    let input_path = match env::args_os().nth(1) {
        Some(ref input) => Path::new(input),
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Usage: netscape-to-univeral <input_path>",
            ));
        }
    }
    .to_owned();

    let mut reader: Box<dyn std::io::Read> = if input_path.as_os_str() == "-" {
        // Get input from stdin
        Box::new(std::io::stdin())
    } else {
        // Get input from file
        match File::open(&input_path) {
            Ok(file) => Box::new(file),
            Err(err) => {
                let msg = format!("{}: {}", input_path.display(), err);
                return Err(Error::new(ErrorKind::InvalidData, msg));
            }
        }
    };

    let node = parse_html().from_utf8().read_from(&mut reader).unwrap();
    to_universal(node);
    Ok(())
}

// 1. Traverse the tree in a breadth-first way
// 2. On each tree level, store parent of the children nodes
// that are to be scanned next.
// 3. For each children node that is a leaf,
// set its list of ancestors, using the ancestor list of the parent
// and adding the parrent to the list.
// Output the node's href and the node's ancestor list as list of tags (URI #tag1 #tag2...)
// 4. For each children that is not a leaf,
// continue the scan (step 2).
fn to_universal(node: NodeRef) {
    if let Some(root) = node.children().find(|n| n.is_element("HTML")) {
        if let Some(body) = root.children().find(|child| child.is_element("BODY")) {
            if let Some(content) = body.children().find(|child| child.is_element("DL")) {
                for item in content.children() {
                    to_universal_rec(&item, vec![]);
                }
            } else {
                panic!("No children found! Are you sure you passed the right file?");
            }
        }
    }
}

fn to_universal_rec(node: &NodeRef, mut ancestors: Vec<String>) {
    if node.is_element("DT") {
        // See if the node is a bookmark
        if let Some(node_a) = node.children().find(|n| n.is_element("A")) {
            if let Some(attribute) = node_a.select_attribute("HREF") {
                println!("{} {}", attribute.value, ancestors.join(" "));
                return;
            }
        }

        // At this point we assume that the node is not a bookmark
        // and so we verify whether the node is a folder
        if let Some(node_h3) = node.children().find(|n| n.is_element("H3")) {
            let title = node_h3.text_contents();
            // Add title to list of ancestors for the children nodes to come
            ancestors.push(format!("#{}", title.replace(' ', "-")));
            // Look for children
            for sibling in node_h3.following_siblings() {
                if sibling.is_element("DL") {
                    for child in sibling.children() {
                        to_universal_rec(&child, ancestors.clone());
                    }
                }
            }
        }
    }
}
