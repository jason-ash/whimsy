use crate::traits::Traverse;
use std::collections::VecDeque;

pub struct BreadthFirstIterator<'a, C, T>
where
    C: Traverse<'a, T>,
    T: 'a,
{
    collection: &'a C,
    queue: VecDeque<&'a T>,
}

impl<'a, C, T> BreadthFirstIterator<'a, C, T>
where
    C: Traverse<'a, T>,
    T: 'a,
{
    pub fn new(collection: &'a C, start: &'a T) -> Self {
        let mut queue = VecDeque::default();
        queue.push_back(start);
        Self { collection, queue }
    }
}

impl<'a, C, T> Iterator for BreadthFirstIterator<'a, C, T>
where
    C: Traverse<'a, T>,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.queue.pop_front() {
            for child in self.collection.children(item) {
                self.queue.push_back(child);
            }
            Some(item)
        } else {
            None
        }
    }
}
