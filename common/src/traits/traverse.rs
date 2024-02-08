use crate::iter::{BreadthFirstIterator, DepthFirstIterator, TraverseByIterator};
use std::cmp::Ordering;

pub trait Hierarchy<T> {
    fn children(&self, item: &T) -> Vec<&T>;
}

pub trait Traverse<'a, T: 'a>: Sized + Hierarchy<T> {
    fn breadth_first_iter(&'a self, start: &'a T) -> BreadthFirstIterator<'a, Self, T> {
        BreadthFirstIterator::new(self, start)
    }

    fn depth_first_iter(&'a self, start: &'a T) -> DepthFirstIterator<'a, Self, T> {
        DepthFirstIterator::new(self, start)
    }

    fn traverse_by<F>(&'a self, start: &'a T, evaluate: F) -> TraverseByIterator<'a, Self, T, F>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        TraverseByIterator::new(self, start, evaluate)
    }
}

impl<'a, T, U: 'a> Traverse<'a, U> for T where T: Hierarchy<U> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Tree(usize, &'static [Tree]);

    impl Hierarchy<Tree> for Tree {
        fn children(&self, item: &Tree) -> Vec<&Tree> {
            item.1.iter().collect()
        }
    }

    const TREE: Tree = Tree(
        0,
        &[
            Tree(1, &[Tree(3, &[]), Tree(4, &[])]),
            Tree(
                2,
                &[
                    Tree(5, &[Tree(7, &[]), Tree(8, &[])]),
                    Tree(6, &[Tree(9, &[])]),
                ],
            ),
        ],
    );

    #[test]
    fn breadth_first() {
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let actual = TREE
            .breadth_first_iter(&TREE) // TODO this is the whole tree, not just the first node...
            .map(|node| node.0)
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn depth_first() {
        let expected = vec![0, 1, 3, 4, 2, 5, 7, 8, 6, 9];
        let actual = TREE
            .depth_first_iter(&TREE) // TODO this is the whole tree, not just the first node...
            .map(|node| node.0)
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn traverse_by() {
        let expected = vec![0, 2, 6, 9];
        let evaluate = |a: &Tree, b: &Tree| a.0.cmp(&b.0);
        let actual = TREE
            .traverse_by(&TREE, evaluate) // TODO this is the whole tree, not just the first node...
            .map(|node| node.0)
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}
