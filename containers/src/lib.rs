use std::ptr::NonNull;
use std::alloc;

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

    fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4)
                .expect("Could not allocate");
            // SAFETY: the layout is hardcoded to be 4 * size_of<T> and
            // size_of<T> is > 0
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could not allocate");
            // SAFETY: ptr is non-null and enough space has just been
            // allocated for item
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self.len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Can't reach memory location");
            assert!(offset < isize::MAX as usize);
            unsafe { self.ptr.as_ptr().add(self.len).write(item) };
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(align - (size % align)).expect("Can't allocate");
            let new_capacity = self.capacity.checked_mul(2)
                .expect("Capacity wrapped");
            let ptr = unsafe {
                let layout = alloc::Layout::
                    from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(
                    self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not allocate");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }

    fn insert(&mut self, item: T, index: usize) {
        assert!(index >= 0);
        assert!(index <= self.len);
        if self.capacity == 0 {
            self.push(item);
        } else if self.len < self.capacity {
            let offset = self.len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Can't reach memory location");
            assert!(offset < isize::MAX as usize);
            unsafe {
                self.len += 1;
                let mut mut_item = item;
                for index in index..self.len {
                    let save = std::ptr::read(self.ptr.as_ptr().add(index));
                    self.ptr.as_ptr().add(index).write(mut_item);
                    mut_item = save;
                }
            }
        } else {
            debug_assert!(self.len == self.capacity);
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(align - (size % align)).expect("Can't allocate");
            let new_capacity = self.capacity.checked_mul(2)
                .expect("Capacity wrapped");
            unsafe {
                let layout = alloc::Layout::
                    from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(
                    self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not allocate");
                self.ptr = ptr;
                self.len += 1;
                self.capacity = new_capacity;
                let mut mut_item = item;
                for index in index..self.len {
                    let save = std::ptr::read(self.ptr.as_ptr().add(index));
                    self.ptr.as_ptr().add(index).write(mut_item);
                    mut_item = save;
                }
            };
        }
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        unsafe { Some(&*self.ptr.as_ptr().add(index)) }
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        unsafe {
            self.len -= 1;
            Some(std::ptr::read(self.ptr.as_ptr().add(self.len)))
        }
    }

    fn remove(&mut self, index: usize) -> T {
        assert!(index >= 0);
        assert!(index < self.len);
        unsafe {
            self.len -= 1; 
            let value = std::ptr::read(self.ptr.as_ptr().add(index));
            for index in index..self.len {
                let write_val = std::ptr::read(self.ptr.as_ptr().add(index + 1));
                std::ptr::write(self.ptr.as_ptr().add(index), write_val);
            }                
            value
        }
    }

    // append, retain and insert
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        println!("drop");
        if self.ptr != NonNull::dangling() {
            unsafe {
                std::ptr::drop_in_place(
                std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
                let layout = alloc::Layout::from_size_align_unchecked(
                    std::mem::size_of::<T>(), std::mem::align_of::<T>());
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    struct Test;

    #[test]
    fn test_myvec() {
        let mut vec: MyVec<usize> = MyVec::new();
        vec.push(4);
        vec.push(2);
        vec.push(1);
        vec.push(5);
        vec.push(6);
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.get(2), Some(&1));
        assert_eq!(vec.capacity(), 8);
    }

    #[test]
    fn test_string() {
        let mut vec: MyVec<String> = MyVec::new();
        vec.push(String::from("Hallo"));
        vec.push(String::from("ik"));
        vec.push(String::from("ben"));
        vec.push(String::from("Daan"));
        vec.push(String::from("!"));
        assert_eq!(vec.len(), 5);
        let string = String::from("Daan");
        assert_eq!(vec.get(3), Some(&string));
        assert_eq!(vec.capacity(), 8);
    }

    #[test]
    #[should_panic]
    fn test_struct() {
        let mut vec: MyVec<Test> = MyVec::new();
        vec.push(Test);
    }

    #[test]
    fn test_pop() {
        let mut vec: MyVec<usize> = MyVec::new();
        vec.push(4);
        vec.push(2);
        vec.push(1);
        vec.push(5);
        vec.push(6);
        assert_eq!(vec.pop(), Some(6));
        assert_eq!(vec.pop(), Some(5));
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.pop(), Some(4));
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_remove() {
        let mut vec: MyVec<usize> = MyVec::new();
        vec.push(4);
        vec.push(2);
        vec.push(1);
        vec.push(5);
        vec.push(6);
        assert_eq!(vec.remove(2), 1);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec.remove(0), 4);
        assert_eq!(vec.remove(2), 6);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.remove(0), 2);
        assert_eq!(vec.remove(0), 5);
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_insert() {
        let mut vec: MyVec<usize> = MyVec::new();
        vec.insert(1, 0);
        vec.insert(4, 0);
        vec.insert(2, 1);
        vec.insert(0, 2);
        vec.insert(8, 2);
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.get(2), Some(&8));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(0), Some(&4));
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.remove(0), 4);
        assert_eq!(vec.len(), 4);
    }
}
