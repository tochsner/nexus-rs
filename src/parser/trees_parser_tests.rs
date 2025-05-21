#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use indextree::Arena;

    use crate::{
        lexer::{lexer::Lexer, tokens::Tokens},
        parser::parser::Parser,
        types::{
            nexus::NexusBlock,
            tree::{Tree, TreeNode},
        },
    };

    impl<'a> Tree<'a> {
        pub fn new(name: &'a str, rooted: bool) -> Tree<'a> {
            Tree {
                tree: Arena::new(),
                name,
                rooted,
            }
        }
    }

    #[test]
    fn test_simplest_tree_block_with_one_node() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=1;
            TAXLABELS Apes;
        END;

        BEGIN trees;
            TREE t1 = Apes;
        END;
        ";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let mut expected_tree = Tree::new("t1", false);
        expected_tree.tree.new_node(TreeNode::new_leaf("Apes"));

        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(HashMap::new(), vec![expected_tree]))
        );
    }

    #[test]
    fn test_simplest_trees_block() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=3;
            TAXLABELS Apes Humans Gorillas;
        END;

        BEGIN trees;
            TREE t1 = ((Apes, Humans), Gorillas);
        END;
        ";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let mut expected_tree = Tree::new("t1", false);

        let apes = expected_tree.tree.new_node(TreeNode::new_leaf("Apes"));
        let humans = expected_tree.tree.new_node(TreeNode::new_leaf("Humans"));
        let apes_humans = expected_tree.tree.new_node(TreeNode::new_internal());
        let gorillas = expected_tree.tree.new_node(TreeNode::new_leaf("Gorillas"));
        let apes_humans_gorillas = expected_tree.tree.new_node(TreeNode::new_root());

        apes_humans_gorillas.append(apes_humans, &mut expected_tree.tree);
        apes_humans_gorillas.append(gorillas, &mut expected_tree.tree);

        apes_humans.append(apes, &mut expected_tree.tree);
        apes_humans.append(humans, &mut expected_tree.tree);

        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(HashMap::new(), vec![expected_tree]))
        );
    }

    #[test]
    fn test_multiple_simplest_trees_block() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=3;
            TAXLABELS Apes Humans Gorillas;
        END;

        BEGIN trees;
            TREE t1 = ((Apes, Humans), Gorillas);
            TREE t2 = (Apes, (Humans, Gorillas));
        END;
        ";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);
        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let mut t1_expected_tree = Tree::new("t1", false);

        let t1_apes = t1_expected_tree.tree.new_node(TreeNode::new_leaf("Apes"));
        let t1_humans = t1_expected_tree.tree.new_node(TreeNode::new_leaf("Humans"));
        let t1_apes_humans = t1_expected_tree.tree.new_node(TreeNode::new_internal());
        let t1_gorillas = t1_expected_tree
            .tree
            .new_node(TreeNode::new_leaf("Gorillas"));
        let t1_apes_humans_gorillas = t1_expected_tree.tree.new_node(TreeNode::new_root());

        t1_apes_humans_gorillas.append(t1_apes_humans, &mut t1_expected_tree.tree);
        t1_apes_humans_gorillas.append(t1_gorillas, &mut t1_expected_tree.tree);

        t1_apes_humans.append(t1_apes, &mut t1_expected_tree.tree);
        t1_apes_humans.append(t1_humans, &mut t1_expected_tree.tree);

        let mut t2_expected_tree = Tree::new("t2", false);

        let t2_apes = t2_expected_tree.tree.new_node(TreeNode::new_leaf("Apes"));
        let t2_humans = t2_expected_tree.tree.new_node(TreeNode::new_leaf("Humans"));
        let t2_gorillas = t2_expected_tree
            .tree
            .new_node(TreeNode::new_leaf("Gorillas"));
        let t2_humans_gorillas = t2_expected_tree.tree.new_node(TreeNode::new_internal());
        let t2_apes_humans_gorillas = t2_expected_tree.tree.new_node(TreeNode::new_root());

        t2_apes_humans_gorillas.append(t2_apes, &mut t2_expected_tree.tree);
        t2_apes_humans_gorillas.append(t2_humans_gorillas, &mut t2_expected_tree.tree);

        t2_humans_gorillas.append(t2_humans, &mut t2_expected_tree.tree);
        t2_humans_gorillas.append(t2_gorillas, &mut t2_expected_tree.tree);

        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(
                HashMap::new(),
                vec![t1_expected_tree, t2_expected_tree]
            ))
        );
    }

    #[test]
    fn test_trees_block_with_lengths() {
        let text = "#NEXUS
        BEGIN taxa;
            DIMENSIONS ntax=3;
            TAXLABELS Apes Humans Gorillas;
        END;

        BEGIN trees;
            TREE t1 = ((Apes: 1.0123, Humans:2):0.10, Gorillas: 2.5e-3);
        END;
        ";
        let lexer = Lexer::new(text);
        let tokens = Tokens::new(&lexer);

        let mut parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let mut expected_tree = Tree::new("t1", false);

        let apes = expected_tree
            .tree
            .new_node(TreeNode::new_leaf("Apes").with_length(1.0123));
        let humans = expected_tree
            .tree
            .new_node(TreeNode::new_leaf("Humans").with_length(2.0));
        let apes_humans = expected_tree
            .tree
            .new_node(TreeNode::new_internal().with_length(0.10));
        let gorillas = expected_tree
            .tree
            .new_node(TreeNode::new_leaf("Gorillas").with_length(0.0025));
        let apes_humans_gorillas = expected_tree.tree.new_node(TreeNode::new_root());

        apes_humans_gorillas.append(apes_humans, &mut expected_tree.tree);
        apes_humans_gorillas.append(gorillas, &mut expected_tree.tree);

        apes_humans.append(apes, &mut expected_tree.tree);
        apes_humans.append(humans, &mut expected_tree.tree);

        assert_eq!(
            result.blocks.get(1),
            Some(&NexusBlock::TreesBlock(HashMap::new(), vec![expected_tree]))
        );
    }
}
