use crate::iter::{BreadthFirstIterator, DepthFirstIterator, TraverseByIterator};
use std::cmp::Ordering;

pub trait Traverse<'a, T: 'a>
where
    Self: Sized,
{
    type DepthFirstIterator: Iterator<Item = &'a T>;
    type TraverseByIterator: Iterator<Item = &'a T>;

    fn children(&self, item: &T) -> Vec<&T>;

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
