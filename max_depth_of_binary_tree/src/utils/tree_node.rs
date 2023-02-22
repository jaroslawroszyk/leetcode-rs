pub(crate) type NodePtr = Option<Box<TreeNode>>;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: NodePtr,
    pub right: NodePtr,
}

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
