use super::{Node, NodeId};

pub struct ArenaTree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> ArenaTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, data: T) -> Option<NodeId> {
        todo!()
    }

    pub fn get(&self, id: NodeId) -> Option<&Node<T>> {
        todo!()
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
        todo!()
    }

    pub fn get_id(&self, node: &Node<T>) -> Option<NodeId> {
        todo!()
    }
}

impl<T> Default for ArenaTree<T> {
    fn default() -> Self {
        Self {
            nodes: Vec::default(),
        }
    }
}
