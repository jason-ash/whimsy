#[derive(Debug, Default)]
pub struct Tree<T> {
    nodes: Vec<T>,
    parents: Vec<usize>,
    children: Vec<Vec<usize>>,
}

impl<T> Tree<T> {
    pub fn get_node(&self, node_id: usize) -> Option<&T> {
        self.nodes.get(node_id)
    }

    pub fn get_node_mut(&mut self, node_id: usize) -> Option<&mut T> {
        self.nodes.get_mut(node_id)
    }

    pub fn get_parent_id(&self, node_id: usize) -> Option<&usize> {
        self.parents.get(node_id)
    }

    pub fn get_parent_node(&self, node_id: usize) -> Option<&T> {
        self.get_parent_id(node_id)
            .and_then(|&id| self.nodes.get(id))
    }

    pub fn get_children_ids(&self, node_id: usize) -> Option<&[usize]> {
        self.children.get(node_id).map(Vec::as_slice)
    }

    pub fn get_children_nodes(&self, node_id: usize) -> Option<Vec<&T>> {
        self.get_children_ids(node_id)
            .and_then(|ids| ids.iter().map(|&id| self.nodes.get(id)).collect())
    }
}
