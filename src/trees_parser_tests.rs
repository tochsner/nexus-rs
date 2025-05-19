#[cfg(test)]
mod tests {
    use std::{collections::HashMap, process::exit};

    use crate::{
        lexer::Lexer,
        nexus::NexusBlock,
        parser::Parser,
        tree::{Tree, TreeNode},
    };

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
        let mut parser = Parser::new(lexer);
        let result = parser.parse().unwrap();

        let mut expected_tree = Tree::new("t1", false);

        let apes_humans_gorillas = expected_tree.tree.new_node(TreeNode::new_root());
        let apes_humans = expected_tree.tree.new_node(TreeNode::new_internal());
        let apes = expected_tree.tree.new_node(TreeNode::new_leaf("Apes"));
        let humans = expected_tree.tree.new_node(TreeNode::new_leaf("Humans"));
        let gorillas = expected_tree.tree.new_node(TreeNode::new_leaf("Gorillas"));

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
