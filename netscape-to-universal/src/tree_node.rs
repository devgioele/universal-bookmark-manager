use crate::bookmark::Bookmark;
use crate::folder::Folder;
use kuchiki::NodeRef;
use std::fs::File;

#[derive(Clone, Debug)]
pub enum TreeNode {
    BookmarkNode(Bookmark),
    FolderNode(Folder),
}

impl TreeNode {
    pub fn from_node(
        node: &NodeRef,
        ancestors: Vec<String>,
        output_file: &mut File,
    ) -> Option<Self> {
        // Attempt parsing as a bookmark, then as a folder
        if let Some(bookmark) = Bookmark::from_node(node, ancestors.clone(), output_file) {
            Some(TreeNode::BookmarkNode(bookmark))
        } else {
            Folder::from_node(node, ancestors, output_file).map(TreeNode::FolderNode)
        }
    }
}
