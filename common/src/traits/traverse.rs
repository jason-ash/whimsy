pub trait Traverse<'a, T: 'a> {
    type BreadthFirstIterator: Iterator<Item = &'a T>;
    type DepthFirstIterator: Iterator<Item = &'a T>;
    type TraverseByIterator: Iterator<Item = &'a T>;

    fn children(&self, item: &T) -> Vec<&T>;

    fn breadth_first_iter(&self, start: &T) -> Self::BreadthFirstIterator;

    fn depth_first_iter(&self, start: &T) -> Self::DepthFirstIterator;

    fn traverse_by<F>(&self, start: &T, evaluate: F) -> Self::TraverseByIterator
    where
        F: Fn(&T, &T) -> std::cmp::Ordering;
}
