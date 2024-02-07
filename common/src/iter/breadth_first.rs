use crate::traits::Traverse;

pub struct BreadthFirstIterator<'a, C, T>
where
    C: Traverse<'a, T>,
    T: 'a,
{
    collection: &'a C,
    queue: Vec<&'a T>,
}

impl<'a, C, T> Iterator for BreadthFirstIterator<'a, C, T>
where
    C: Traverse<'a, T>,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.queue.pop() {
            let children = self.collection.children(item);
            self.queue.extend(children);
            return Some(item);
        }
        None
    }
}
