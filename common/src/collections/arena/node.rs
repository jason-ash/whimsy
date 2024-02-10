pub struct NodeId(usize);

pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}
