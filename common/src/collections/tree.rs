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

    pub fn get_parent_id(&self, node_id: usize) -> Option<&usize> {
        self.parents.get(node_id)
    }

    pub fn get_children(&self, node_id: usize) -> Option<&[usize]> {
        self.children.get(node_id).map(Vec::as_slice)
    }
}
