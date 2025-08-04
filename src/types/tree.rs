use indextree::Arena;

#[derive(PartialEq, Debug)]
pub enum TreeNode {
    Leaf {
        label: String,
        taxon: String,
        length: Option<f64>,
    },
    InternalNode {
        label: Option<String>,
        length: Option<f64>,
    },
    Root {
        label: Option<String>,
    },
}

impl TreeNode {
    pub fn new_leaf(taxon: String) -> Self {
        TreeNode::Leaf {
            label: taxon.to_string(),
            taxon,
            length: None,
        }
    }

    pub fn new_internal() -> Self {
        TreeNode::InternalNode {
            label: None,
            length: None,
        }
    }

    pub fn new_root() -> Self {
        TreeNode::Root { label: None }
    }

    pub fn with_length(self, length: f64) -> Self {
        match self {
            TreeNode::Leaf { label, taxon, .. } => TreeNode::Leaf {
                label,
                taxon,
                length: Some(length),
            },
            TreeNode::InternalNode { label, .. } => TreeNode::InternalNode {
                label,
                length: Some(length),
            },
            TreeNode::Root { label } => TreeNode::Root { label },
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Tree {
    pub tree: Arena<TreeNode>,
    pub name: String,
    pub rooted: bool,
}
