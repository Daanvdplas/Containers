use std::ptr::NonNull;
use std::alloc;
use std::fmt;
use crate::myiter::*;

pub mod myiter;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::<T>::dangling(), 
            len: 0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: T) {
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

    pub fn insert(&mut self, item: T, index: usize) {
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
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        unsafe { Some(&*self.ptr.as_ptr().add(index)) }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        unsafe {
            self.len -= 1;
            Some(std::ptr::read(self.ptr.as_ptr().add(self.len)))
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
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

    pub fn append(&mut self, other: &mut MyVec<T>) {
        let len = self.len.checked_add(other.len).expect("Can't reach memory location");
        assert!(len < isize::MAX as usize);
        if len <= self.capacity {
            unsafe {
                for i in 0..other.len {
                    self.ptr.as_ptr().add(self.len + i).write(
                        std::ptr::read(other.ptr.as_ptr().add(i)));
                }
            }
            self.len = len;
        } else {
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(align - (size % align)).expect("Can't allocate");
            let mut new_capacity = self.capacity;
            while new_capacity < len {
                new_capacity = new_capacity.checked_mul(2)
                    .expect("Capacity wrapped");
            }
            let ptr = unsafe {
                let layout = alloc::Layout::
                    from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(
                    self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not allocate");
                for i in 0..other.len {
                    self.ptr.as_ptr().add(self.len + i).write(
                        std::ptr::read(other.ptr.as_ptr().add(i)));
                }
                ptr
            };
            self.ptr = ptr;
            self.len = len;
            other.len = 0;
            self.capacity = new_capacity;
        }
    }
    
    pub fn into_iter(self) -> MyIntoIter<T> {
        myiter::MyIntoIter(self)
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
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

impl<T> Clone for MyVec<T> {
    fn clone (&self) -> Self {
        let layout = alloc::Layout::array::<T>(self.capacity)
            .expect("Could not allocate");
        let ptr = unsafe { alloc::alloc(layout) } as *mut T;
        let ptr = NonNull::new(ptr).expect("Could not allocate");
        unsafe {
            for i in 0..self.len {
                let item = std::ptr::read(self.ptr.as_ptr().add(i));
                ptr.as_ptr().add(i).write(item);
            }
        }
        MyVec {
            ptr: ptr,
            len: self.len,
            capacity: self.capacity,
        }
    }
}

impl<T: fmt::Debug + fmt::Display> fmt::Debug for MyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[");
        unsafe {
            for i in 0..self.len {
                if i == self.len - 1 {
                    write!(f, "\"{}\"", std::ptr::read(self.ptr.as_ptr().add(i)));
                } else {
                    write!(f, "\"{}\", ", std::ptr::read(self.ptr.as_ptr().add(i)));
                }
            }
        }
        write!(f, "]");
        Ok(())
    }
}

