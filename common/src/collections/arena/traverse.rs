use super::{ArenaTree, Node};

pub struct AncestorIterator<'a, T> {
    tree: &'a ArenaTree<T>,
}

impl<'a, T> Iterator for AncestorIterator<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct BreadthFirstIterator<'a, T> {
    tree: &'a ArenaTree<T>,
}

impl<'a, T> Iterator for BreadthFirstIterator<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
