use crate::node_ref_ext::*;
use kuchiki::{parse_html, traits::TendrilSink, NodeRef};
use std::{
    fs::File,
    io::{self, Error, ErrorKind, Read, Write},
    path::PathBuf,
};

mod node_ref_ext;

pub fn read_path(input_path: PathBuf) -> Result<Box<dyn Read>, Error> {
    if input_path.as_os_str() == "-" {
        // Get input from stdin
        Ok(Box::new(io::stdin()) as Box<dyn Read>)
    } else {
        // Get input from file
        match File::open(&input_path) {
            Ok(file) => Ok(Box::new(file) as Box<dyn Read>),
            Err(err) => {
                let msg = format!("{}: {}", input_path.display(), err);
                Err(Error::new(ErrorKind::InvalidData, msg))
            }
        }
    }
}

pub fn convert<R: Read, W: Write>(input: &mut R, output: &mut W) -> Result<(), Error> {
    let node = parse_html().from_utf8().read_from(input).unwrap();
    to_universal(node, output)
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
fn to_universal(node: NodeRef, output: &mut dyn Write) -> Result<(), Error> {
    if let Some(root) = node.children().find(|n| n.is_element("HTML")) {
        if let Some(body) = root.children().find(|child| child.is_element("BODY")) {
            if let Some(content) = body.children().find(|child| child.is_element("DL")) {
                for item in content.children() {
                    to_universal_rec(&item, vec![], output)?;
                }
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid file format: missing content element DL.",
                ));
            }
        }
    }
    Ok(())
}

fn to_universal_rec(
    node: &NodeRef,
    mut ancestors: Vec<String>,
    output: &mut dyn Write,
) -> Result<(), Error> {
    if node.is_element("DT") {
        // See if the node is a bookmark
        if let Some(node_a) = node.children().find(|n| n.is_element("A")) {
            if let Some(attribute) = node_a.select_attribute("HREF") {
                writeln!(
                    output,
                    "{}{}{}",
                    attribute.value,
                    if ancestors.is_empty() { "" } else { " " },
                    ancestors.join(" ")
                )?;
                return Ok(());
            }
        }

        // At this point we know that the node is not a bookmark
        // and so we verify whether the node is a folder
        if let Some(node_h3) = node.children().find(|n| n.is_element("H3")) {
            let title = node_h3.text_contents();
            // Add title to list of ancestors for the children nodes to come
            ancestors.push(format!("#{}", title.replace(' ', "-")));
            // Look for children
            for sibling in node_h3.following_siblings() {
                if sibling.is_element("DL") {
                    for child in sibling.children() {
                        to_universal_rec(&child, ancestors.clone(), output)?;
                    }
                }
            }
        }
    }
    Ok(())
}
