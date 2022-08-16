use std::ptr::NonNull;

struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        Self {
            ptr: NonNull::<T>::dangling(), 
            len: 0,
            capacity: 0,
        }
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
    
    fn len(&self) -> usize {
        self.len
    }

    push(&self) -> 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_myvec() {
        let mut vec: MyVec<usize> = MyVec::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);
    }
}
