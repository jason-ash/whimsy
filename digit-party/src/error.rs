use petgraph::graph::NodeIndex;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidNode(NodeIndex),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidNode(node_id) => write!(f, "Invalid node: {:?}.", node_id),
        }
    }
}

impl std::error::Error for Error {}
