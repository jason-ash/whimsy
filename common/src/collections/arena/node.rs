#[derive(Debug, Clone)]
pub struct NodeId(usize);

pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> Node<T> {
    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.parent.clone()
    }

    pub fn children(&self) -> &[NodeId] {
        self.children.as_slice()
    }
}
