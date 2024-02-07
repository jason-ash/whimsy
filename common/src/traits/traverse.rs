pub trait Traverse<T> {
    fn children(&self, item: &T) -> Vec<&T>;

    fn breadth_first_iter(&self, start: &T) -> Vec<&T>;

    fn depth_first_iter(&self, start: &T) -> Vec<&T>;

    fn traverse_by<F>(&self, start: &T, evaluate: F) -> Vec<&T>
    where
        F: Fn(&T, &T) -> std::cmp::Ordering;
}
