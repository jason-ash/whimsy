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
