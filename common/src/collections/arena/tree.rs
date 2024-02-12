use super::{Node, NodeId};
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, data: T, parent: Option<NodeId>) -> Option<NodeId> {
        let id = NodeId(self.nodes.len());
        parent
            .and_then(|id| self.get_mut(id))
            .map(|node| node.children.push(id));
        self.nodes.push(Node::new(data, parent));
        Some(id)
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

impl<T: Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tree").field("nodes", &self.nodes).finish()
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
    fn validate_sample_tree() {
        let mut tree = Tree::<u8>::default();
        let node0 = tree.insert(0, None);
        let node1 = tree.insert(1, node0);
        let node2 = tree.insert(2, node0);
        let _node3 = tree.insert(3, node1);
        let _node4 = tree.insert(4, node1);
        let node5 = tree.insert(5, node2);
        let node6 = tree.insert(6, node2);
        let _node7 = tree.insert(7, node5);
        let _node8 = tree.insert(8, node6);
        let node9 = tree.insert(9, node6);

        let expected = vec![9, 6, 2, 0];
        let ancestors = node9
            .unwrap()
            .ancestors(&tree)
            .filter_map(|id| tree.get(id).map(Node::data).cloned())
            .collect::<Vec<_>>();
        assert_eq!(ancestors, expected);

        let expected = vec![1, 2];
        let children = node0
            .and_then(|id| tree.get(id))
            .map(Node::children)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|&id| tree.get(id).map(Node::data).cloned())
            .collect::<Vec<_>>();
        assert_eq!(children, expected);

        let expected = vec![0, 2, 6, 9];
        let max_value_children = node0
            .unwrap()
            .traverse_by(&tree, |a, b| a.data().cmp(b.data()))
            .filter_map(|id| tree.get(id).map(Node::data).cloned())
            .collect::<Vec<_>>();
        assert_eq!(max_value_children, expected);
    }
}
