use super::{traverse::TraverseByIterator, AncestorIterator, Node, Tree};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct NodeId(pub(super) usize);

impl NodeId {
    pub fn ancestors<'a, T>(self, tree: &'a Tree<T>) -> AncestorIterator<'a, T> {
        AncestorIterator::new(tree, self)
    }

    pub fn traverse_by<'a, T, F>(
        self,
        tree: &'a Tree<T>,
        evaluate: F,
    ) -> TraverseByIterator<'a, T, F>
    where
        F: Fn(&Node<T>, &Node<T>) -> Ordering,
    {
        todo!()
    }
}
