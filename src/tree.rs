use indextree::Arena;

#[derive(PartialEq, Debug)]
pub enum TreeNode<'a> {
    Leaf {
        label: &'a str,
        taxon: &'a str,
        length: Option<f64>,
    },
    InternalNode {
        label: Option<&'a str>,
        length: Option<f64>,
    },
    Root {
        label: Option<&'a str>,
    },
}

impl<'a> TreeNode<'a> {
    pub fn new_leaf(taxon: &'a str) -> Self {
        TreeNode::Leaf {
            label: taxon,
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
pub struct Tree<'a> {
    pub tree: Arena<TreeNode<'a>>,
    pub name: &'a str,
    pub rooted: bool,
}

impl<'a> Tree<'a> {
    pub fn new(name: &'a str, rooted: bool) -> Tree<'a> {
        Tree {
            tree: Arena::new(),
            name,
            rooted,
        }
    }
}
