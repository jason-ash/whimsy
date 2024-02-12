use std::fmt::Debug;

use super::NodeId;

pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    pub(super) children: Vec<NodeId>,
}

impl<T> Node<T> {
    pub(super) fn new(data: T, parent: Option<NodeId>) -> Self {
        Self {
            data,
            parent,
            children: Vec::new(),
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    pub fn children(&self) -> &[NodeId] {
        self.children.as_slice()
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .finish()
    }
}
