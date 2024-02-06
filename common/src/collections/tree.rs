#[derive(Debug, Default)]
pub struct Tree<T> {
    nodes: Vec<T>,
    parents: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
}

impl<T> Tree<T> {
    pub fn get_node(&self, node_id: usize) -> Option<&T> {
        self.nodes.get(node_id)
    }

    pub fn get_node_mut(&mut self, node_id: usize) -> Option<&mut T> {
        self.nodes.get_mut(node_id)
    }

    pub fn get_parent_id(&self, node_id: usize) -> Option<usize> {
        self.parents.get(node_id)?.to_owned()
    }

    pub fn get_parent_node(&self, node_id: usize) -> Option<&T> {
        self.parents.get(node_id)?.and_then(|id| self.nodes.get(id))
    }

    pub fn get_children_ids(&self, node_id: usize) -> Option<&[usize]> {
        self.children.get(node_id).map(Vec::as_slice)
    }

    pub fn get_children_nodes(&self, node_id: usize) -> Option<Vec<&T>> {
        self.children
            .get(node_id)
            .and_then(|ids| ids.iter().map(|&id| self.nodes.get(id)).collect())
    }

    pub fn add_node(&mut self, node: T, parent_id: Option<usize>) -> Option<usize> {
        let node_id = self.nodes.len();

        // validate the node's parent, then add the new node to the parent's children array.
        if let Some(parent_id) = parent_id {
            self.children.get_mut(parent_id)?.push(node_id);
        }

        // add the node to the nodes array, add the parent_id and an empty children vector.
        self.nodes.push(node);
        self.parents.push(parent_id);
        self.children.push(Vec::new());

        Some(node_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Node(u64);

    #[test]
    fn get_node_after_insert() {
        let mut tree = Tree::default();
        let node_id = tree.add_node(Node(0), None).unwrap();
        let node_ref = tree.get_node(0);
        assert!(matches!(Some(&Node(0)), node_ref));
    }

    fn validate_simple_tree() {
        let mut tree = Tree::default();
        let parent_id = tree.add_node(Node(0), None);
        let child1_id = tree.add_node(Node(1), parent_id);
        let child2_id = tree.add_node(Node(2), parent_id);
        let child3_id = tree.add_node(Node(3), child1_id);
        let child4_id = tree.add_node(Node(4), child2_id);
        let child5_id = tree.add_node(Node(5), child2_id);
    }
}
