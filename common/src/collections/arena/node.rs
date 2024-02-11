use super::NodeId;

pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
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
