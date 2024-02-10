pub struct NodeId(usize);

pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> Node<T> {
    pub fn data(&self) -> &T {
        todo!()
    }

    pub fn data_mut(&mut self) -> &mut T {
        todo!()
    }

    pub fn parent(&self) -> Option<NodeId> {
        todo!()
    }

    pub fn children(&self) -> &[NodeId] {
        todo!()
    }
}
