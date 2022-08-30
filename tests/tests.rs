use containers::*;
use containers::myiter::MyIterator;

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

#[test]
fn test_append() {
    let mut vec: MyVec<usize> = MyVec::new();
    vec.insert(1, 0);
    vec.insert(4, 0);
    vec.insert(2, 1);
    vec.insert(0, 2);
    vec.insert(8, 2);

    let mut vec2: MyVec<usize> = MyVec::new();
    vec2.push(4);
    vec2.push(2);
    vec2.push(1);
    vec2.push(5);
    vec2.push(6);
    vec.append(&mut vec2);
    assert_eq!(vec.len(), 10);
    assert_eq!(vec2.len(), 0);
    assert_eq!(vec.capacity(), 16);
}

#[test]
fn test_clone() {
    let mut vec: MyVec<usize> = MyVec::new();
    vec.push(4);
    vec.push(2);
    vec.push(1);
    vec.push(5);
    vec.push(6);
    let mut cloned = vec.clone();
    assert_eq!(cloned.len(), 5);
    assert_eq!(cloned.capacity(), 8);
    assert_eq!(cloned.get(2), Some(&1));
    assert_eq!(cloned.pop(), Some(6));
}

#[test]
fn test_iter() {
    let mut vec: MyVec<usize> = MyVec::new();
    vec.push(4);
    vec.push(2);
    vec.push(1);
    vec.push(5);
    vec.push(6);
    let mut iter = vec.into_iter();
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
}
