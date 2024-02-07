use crate::traits::Traverse;
use std::collections::VecDeque;

pub struct BreadthFirstIterator<'a, C, T>
where
    C: Traverse<T> + 'a,
    T: 'a,
{
    collection: &'a C,
    queue: VecDeque<&'a T>,
}

impl<'a, C, T> Iterator for BreadthFirstIterator<'a, C, T>
where
    C: Traverse<T> + 'a,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.queue.pop_front() {
            let children = self.collection.children(item);
            self.queue.extend(children);
            return Some(item);
        }
        None
    }
}
