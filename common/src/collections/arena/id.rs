use std::cmp::Ordering;

use super::{traverse::TraverseByIterator, AncestorIterator, Tree};

#[derive(Debug, Clone, Copy)]
pub struct NodeId(pub(super) usize);

impl NodeId {
    pub fn append<T>(self, child: NodeId, tree: &mut Tree<T>) {
        todo!()
    }

    pub fn ancestors<'a, T>(self, tree: &'a Tree<T>) -> AncestorIterator<'a, T> {
        AncestorIterator::new(tree, self)
    }

    pub fn traverse_by<'a, T, F>(
        self,
        tree: &'a Tree<T>,
        evaluate: F,
    ) -> TraverseByIterator<'a, T, F>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        todo!()
    }
}
