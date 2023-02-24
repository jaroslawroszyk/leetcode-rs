use crate::structures::tree_node_struct::*;

#[allow(dead_code)]
pub fn new_tree_node(val: i32, left: NodePtr, right: NodePtr) -> NodePtr {
    Some(Box::new(TreeNode { val, left, right }))
}

pub fn max_depth(root: &NodePtr) -> i32 {
    match root {
        None => 0,
        Some(node_ptr) => {
            let left_depth = max_depth(&node_ptr.left);
            let right_depth = max_depth(&node_ptr.right);
            left_depth.max(right_depth) + 1
        }
    }
}

#[macro_export]
macro_rules! node {
    ($val: expr) => {
        Some(Box::new(TreeNode {
            val: $val,
            left: None,
            right: None,
        }))
    };
    ($val: expr, $left: expr, $right: expr) => {
        Some(Box::new(TreeNode {
            val: $val,
            left: $left,
            right: $right,
        }))
    };
}

#[cfg(test)]
mod tests {
    use crate::problems::maximum_depth_of_binary_tree::*;

    #[test]
    fn should_return_tree_with_given_tree() {
        let root = node!(3, node!(9), node!(20, node!(15), node!(7)));
        let result = max_depth(&root);
        let expected = 3;
        assert_eq!(result, expected);
    }
}
