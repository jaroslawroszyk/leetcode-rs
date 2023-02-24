pub(crate) type NodePtr = Option<Box<TreeNode>>;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: NodePtr,
    pub right: NodePtr,
}
