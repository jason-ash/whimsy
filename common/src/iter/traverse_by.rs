use crate::traits::Traverse;
use std::{cmp::Ordering, collections::VecDeque};

pub struct TraverseByIterator<'a, C, T, F>
where
    F: Fn(&T, &T) -> Ordering,
    C: Traverse<'a, T>,
    T: 'a,
{
    collection: &'a C,
    queue: VecDeque<&'a T>,
    f: F,
}

impl<'a, C, T, F> Iterator for TraverseByIterator<'a, C, T, F>
where
    F: Fn(&T, &T) -> Ordering,
    C: Traverse<'a, T>,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
