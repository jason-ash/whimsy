use crate::traits::Traverse;
use std::cmp::Ordering;

pub struct TraverseByIterator<'a, C, T, F>
where
    F: Fn(&'a T, &'a T) -> Ordering,
    C: Traverse<'a, T>,
    T: 'a,
{
    collection: &'a C,
    queue: Vec<&'a T>,
    f: F,
}

impl<'a, C, T, F> Iterator for TraverseByIterator<'a, C, T, F>
where
    F: Fn(&'a T, &'a T) -> Ordering,
    C: Traverse<'a, T>,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.queue.pop() {
            if let Some(child) = self
                .collection
                .children(item)
                .into_iter()
                .max_by(|a, b| (self.f)(a, b))
            {
                self.queue.push(child);
            }
            return Some(item);
        }
        None
    }
}
