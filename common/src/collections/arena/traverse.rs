use std::collections::VecDeque;

use super::{ArenaTree, Node, NodeId};

pub struct ChildrenIterator<'a, T> {
    tree: &'a ArenaTree<T>,
    node: Option<NodeId>,
}

impl<'a, T> ChildrenIterator<'a, T> {
    pub fn new(tree: &'a ArenaTree<T>, current: NodeId) -> Self {
        Self {
            tree,
            node: Some(current),
        }
    }
}

impl<'a, T> Iterator for ChildrenIterator<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.node.take()?;
        // self.node = self.tree.get(current).next_sibling()
        Some(current)
    }
}

pub struct AncestorIterator<'a, T> {
    tree: &'a ArenaTree<T>,
    node: Option<NodeId>,
}

impl<'a, T> AncestorIterator<'a, T> {
    pub fn new(tree: &'a ArenaTree<T>, current: NodeId) -> Self {
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

pub struct DepthFirstIterator<'a, T> {
    tree: &'a ArenaTree<T>,
    node: Option<NodeId>,
}
