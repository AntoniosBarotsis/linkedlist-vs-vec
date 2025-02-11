#![allow(unstable_features)]
#![feature(linked_list_remove)]

use std::collections::LinkedList;

pub fn generate(range: usize) -> (LinkedList<i32>, Vec<i32>) {
  let mut list = LinkedList::<i32>::new();
  let mut vec = Vec::<i32>::new();

  for i in 0..range {
    list.push_back(i as i32);
    vec.push(i as i32);
  }

  (list, vec)
}

pub fn remove_ith_list(mut list: LinkedList<i32>, i: usize) -> LinkedList<i32> {
  let _ = list.remove(i);
  list
}

#[allow(clippy::needless_pass_by_value)]
pub fn remove_ith_vec(vec: Vec<i32>, i: usize) -> Vec<i32> {
  let mut left = vec[..i].to_owned();
  let mut right = vec[i + 1..].to_owned();
  left.append(&mut right);
  left
}

#[cfg(test)]
mod tests {
  use crate::{generate, remove_ith_list, remove_ith_vec};

  #[test]
  fn test() {
    let range = 100usize;
    let (list, vec) = generate(range);
    assert!(list.len() == range);
    assert!(vec.len() == range);

    let list_new = remove_ith_list(list, range / 2);
    let vec_new = remove_ith_vec(vec, range / 2);

    assert!(list_new.len() == range - 1);
    assert!(vec_new.len() == range - 1);
  }
}
