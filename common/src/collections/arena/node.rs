use super::{traverse::ChildrenIterator, AncestorIterator, ArenaTree};

#[derive(Debug, Clone, Copy)]
pub struct NodeId(usize);

impl NodeId {
    pub fn ancestors<'a, T>(self, tree: &'a ArenaTree<T>) -> AncestorIterator<'a, T> {
        AncestorIterator::new(tree, self)
    }
}

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
        self.parent
    }

    pub fn children(&self) -> &[NodeId] {
        self.children.as_slice()
    }
}
