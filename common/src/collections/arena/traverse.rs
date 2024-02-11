use std::collections::VecDeque;

use super::{ArenaTree, Node, NodeId};

pub struct ParentIterator<'a, T> {
    tree: &'a ArenaTree<T>,
    node_id: Option<NodeId>,
}

impl<'a, T> Iterator for ParentIterator<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node_id.take()?;
        self.node_id = self.tree.get(current).and_then(Node::parent);
        Some(current)
    }
}

pub struct BreadthFirstIterator<'a, T> {
    tree: &'a ArenaTree<T>,
    node_id: Option<NodeId>,
    queue: VecDeque<NodeId>,
}

impl<'a, T> Iterator for BreadthFirstIterator<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node_id.take()?;
        let _children = self
            .tree
            .get(current)
            .map(Node::children)
            .unwrap_or_else(|| &[])
            .into_iter()
            .for_each(|&node_id| self.queue.push_front(node_id));
        Some(current)
    }
}
