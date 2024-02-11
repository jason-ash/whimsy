use super::{Node, NodeId, Tree};

pub struct AncestorIterator<'a, T> {
    tree: &'a Tree<T>,
    node: Option<NodeId>,
}

impl<'a, T> AncestorIterator<'a, T> {
    pub fn new(tree: &'a Tree<T>, current: NodeId) -> Self {
        Self {
            tree,
            node: Some(current),
        }
    }
}

impl<'a, T> Iterator for AncestorIterator<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node.take()?;
        self.node = self.tree.get(current).and_then(Node::parent);
        Some(current)
    }
}
