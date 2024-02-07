use crate::traits::Traverse;
use std::{cmp::Ordering, collections::VecDeque};

pub struct TraverseByIterator<'a, C, T, F>
where
    F: Fn(&'a T, &'a T) -> Ordering,
    C: Traverse<'a, T>,
    T: 'a,
{
    collection: &'a C,
    queue: VecDeque<&'a T>,
    f: F,
}

impl<'a, C, T, F> TraverseByIterator<'a, C, T, F>
where
    F: Fn(&'a T, &'a T) -> Ordering,
    C: Traverse<'a, T>,
    T: 'a,
{
    pub fn new(collection: &'a C, start: &'a T, f: F) -> Self {
        let mut queue = VecDeque::default();
        queue.push_back(start);
        Self {
            collection,
            queue,
            f,
        }
    }
}

impl<'a, C, T, F> Iterator for TraverseByIterator<'a, C, T, F>
where
    F: Fn(&'a T, &'a T) -> Ordering,
    C: Traverse<'a, T>,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.queue.pop_front() {
            if let Some(child) = self
                .collection
                .children(item)
                .into_iter()
                .max_by(|a, b| (self.f)(a, b))
            {
                self.queue.push_back(child);
            }
            return Some(item);
        }
        None
    }
}
