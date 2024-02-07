use crate::iter::BreadthFirstIterator;

pub trait Traverse<'a, T: 'a>
where
    Self: Sized,
{
    type DepthFirstIterator: Iterator<Item = &'a T>;
    type TraverseByIterator: Iterator<Item = &'a T>;

    fn children(&self, item: &T) -> Vec<&T>;

    fn breadth_first_iter(&'a self, start: &T) -> BreadthFirstIterator<'a, Self, T> {
        BreadthFirstIterator::new(self, start)
    }

    fn depth_first_iter(&self, start: &T) -> Self::DepthFirstIterator;

    fn traverse_by<F>(&self, start: &T, evaluate: F) -> Self::TraverseByIterator
    where
        F: Fn(&T, &T) -> std::cmp::Ordering;
}
