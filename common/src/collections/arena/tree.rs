use super::{Node, NodeId};
use std::ops::{Index, IndexMut};

pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, data: T, parent: Option<NodeId>) -> Option<NodeId> {
        // if the parent is None, then validate we don't already have a root node; otherwise
        // if the parent is Some(id), then validate that the node already exists in the tree.
        match parent {
            Some(id) => {
                if self.get(id).is_none() {
                    return None;
                }
            }
            None => {
                if !self.nodes.is_empty() {
                    return None;
                }
            }
        }

        let node = Node::new(data, parent);
        let node_id = NodeId(self.nodes.len());
        self.nodes.push(node);
        Some(node_id)
    }

    pub fn get(&self, id: NodeId) -> Option<&Node<T>> {
        self.nodes.get(id.0)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id.0)
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            nodes: Vec::default(),
        }
    }
}

impl<T> Index<NodeId> for Tree<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index.0]
    }
}

impl<T> IndexMut<NodeId> for Tree<T> {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.nodes[index.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tree() {
        let mut tree = Tree::<u8>::default();
        let root = tree.insert(0, None);
        let child0_1 = tree.insert(1, root);
        let child0_2 = tree.insert(2, root);
        let child1_3 = tree.insert(3, child0_1);
        let child1_4 = tree.insert(4, child0_1);
        let child2_5 = tree.insert(5, child0_2);
        let child2_6 = tree.insert(6, child0_2);
        let child5_7 = tree.insert(7, child2_5);
        let child5_8 = tree.insert(8, child2_5);
        let child6_9 = tree.insert(8, child2_6);
    }
}
