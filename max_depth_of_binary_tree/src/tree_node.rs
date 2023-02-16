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
