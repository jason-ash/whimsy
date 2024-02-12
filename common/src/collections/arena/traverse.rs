use super::{Node, NodeId, Tree};
use std::cmp::Ordering;

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

pub struct TraverseByIterator<'a, T, F>
where
    F: Fn(&Node<T>, &Node<T>) -> Ordering,
{
    tree: &'a Tree<T>,
    node: Option<NodeId>,
    f: F,
}

impl<'a, T, F> Iterator for TraverseByIterator<'a, T, F>
where
    F: Fn(&Node<T>, &Node<T>) -> Ordering,
{
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node.take()?;
        let k = self.tree.get(current).map(Node::children).map(|children| {
            children
                .into_iter()
                .filter_map(|&id| self.tree.get(id))
                .max_by(|a, b| (self.f)(a, b))
        });

        Some(current)
    }
}
