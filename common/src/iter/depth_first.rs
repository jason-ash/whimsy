use crate::traits::Hierarchy;
use std::collections::VecDeque;

pub struct DepthFirstIterator<'a, C, T>
where
    C: Hierarchy<T>,
    T: 'a,
{
    collection: &'a C,
    queue: VecDeque<&'a T>,
}

impl<'a, C, T> DepthFirstIterator<'a, C, T>
where
    C: Hierarchy<T>,
    T: 'a,
{
    pub fn new(collection: &'a C, start: &'a T) -> Self {
        let mut queue = VecDeque::default();
        queue.push_back(start);
        Self { collection, queue }
    }
}

impl<'a, C, T> Iterator for DepthFirstIterator<'a, C, T>
where
    C: Hierarchy<T>,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.queue.pop_front() {
            todo!();
        }
        None
    }
}
