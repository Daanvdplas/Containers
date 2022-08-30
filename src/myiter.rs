use crate::MyVec;

pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub struct MyIntoIter<T>(pub MyVec<T>);

impl<T> MyIterator for MyIntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len == 0 {
            return None;
        }
        Some(self.0.remove(0))
    }
}
