use crate::tree::Tree;
use kuchiki::{parse_html, traits::TendrilSink};
use std::{fs::File, path::Path};

mod bookmark;
mod folder;
mod node_ref_ext;
mod tree;
mod tree_node;

const INPUT_PATH: &str = "./res/input.html";
const OUTPUT_PATH: &str = "./res/output";

// 1. Traverse the tree in breadth-first way
// 2. On each tree level, store parent of the children nodes
// that are to be scanned next.
// 3. For each children node that is a leaf,
// set its list of ancestors, using the ancestor list of the parent
// and adding the parrent to the list.
// Output the node's href and the node's ancestor list as list of tags (URI #tag1 #tag2...)
// 4. For each children that is not a leaf,
// continue the scan (step 2).
fn main() {
    let input_path = Path::new(INPUT_PATH);
    let mut output_file = File::create(OUTPUT_PATH).unwrap();

    println!("Parsing bookmarks...");
    let tree_result = parse_html()
        .from_utf8()
        .from_file(input_path)
        .map(|node| Tree::from_node(&node, &mut output_file));

    match tree_result {
        Ok(_) => {
            println!("Bookmarks parsed successfully!");
        }
        _ => {
            println!("Bookmarks could not be parsed! Are they saved in an invalid format?");
        }
    }
}
