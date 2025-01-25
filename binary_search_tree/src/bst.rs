// Copyright (C) 2025 BastIsAwesome (bastisawesomeltd@gmail.com)
//
// This file is part of deque.
//
// deque is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// deque is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with deque.  If not, see <https://www.gnu.org/licenses/>.
/// Duplicate values are ignored.
/// Operations on values not in the tree are ignored.
pub struct BinarySearchTree<T: PartialEq + PartialOrd> {
    root: Option<Box<BstNode<T>>>,
}

impl<T: PartialEq + PartialOrd> BinarySearchTree<T> {
    pub fn new(data: Option<T>) -> Self {
        match data {
            Some(data) => Self {
                root: Some(Box::new(BstNode::new(data))),
            },
            None => Self { root: None },
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(BstNode::<T>::new(value)));
            return;
        }

        insert(self.root.as_mut().unwrap(), value);
    }

    pub fn delete(&mut self, value: T) {
        if self.root.is_none() {
            return;
        }

        if self.root.as_ref().unwrap().data == value {
            if self.root.as_ref().unwrap().right.is_none() {
                self.root = self.root.as_mut().unwrap().left.take();
                return;
            }
            if self.root.as_ref().unwrap().left.is_none() {
                self.root = self.root.as_mut().unwrap().right.take();
                return;
            }
            self.root =
                get_successor(self.root.as_mut().unwrap().left.take().unwrap());
        }

        delete(self.root.as_mut().unwrap(), value);
    }

    pub fn contains(&self, value: T) -> bool {
        if self.root.is_none() {
            return false;
        }

        contains(self.root.as_ref().unwrap(), value)
    }

    pub fn min(&self) -> Option<T> {
        if self.root.is_none() {
            return None;
        }

        min(self.root.as_ref().unwrap())
    }

    pub fn max(&self) -> Option<T> {
        if self.root.is_none() {
            return None;
        }

        max(self.root.as_ref().unwrap())
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn size(&self) -> usize {
        if self.root.is_none() {
            return 0;
        }

        size(self.root.as_ref().unwrap())
    }

    pub fn height(&self) -> usize {
        if self.root.is_none() {
            return 0;
        }

        height(self.root.as_ref().unwrap())
    }

    pub fn clear(&mut self) {
        self.root = None;
    }
}

fn get_successor<T: PartialEq + PartialOrd>(
    node: Box<BstNode<T>>,
) -> Option<Box<BstNode<T>>> {
    if node.right.is_none() {
        return Some(node);
    }

    get_successor(node.right.unwrap())
}

fn insert<T: PartialEq + PartialOrd>(node: &mut Box<BstNode<T>>, value: T) {
    if node.data == value {
        return;
    }

    if value < node.data {
        match &mut node.left {
            Some(node) => insert(node, value),
            None => node.left = Some(Box::new(BstNode::<T>::new(value))),
        }
    } else {
        match &mut node.right {
            Some(node) => insert(node, value),
            None => node.right = Some(Box::new(BstNode::<T>::new(value))),
        }
    }
}

/// Assumes the current node has already been checked and does not match.
fn delete<T: PartialEq + PartialOrd>(node: &mut Box<BstNode<T>>, value: T) {
    if value < node.data {
        if node.left.is_none() {
            // Value does not exist.
            return;
        }
        if node.left.as_ref().unwrap().data == value {
            node.left = None;
            return;
        }
        delete(node.left.as_mut().unwrap(), value);
        return;
    }

    if node.right.is_none() {
        // Value does not exist.
        return;
    }

    if node.right.as_ref().unwrap().data == value {
        node.right = None;
        return;
    }

    delete(node.right.as_mut().unwrap(), value);
}

pub fn contains<T: PartialEq + PartialOrd>(
    node: &Box<BstNode<T>>,
    value: T,
) -> bool {
    todo!();
}

pub fn min<T: PartialEq + PartialOrd>(node: &Box<BstNode<T>>) -> Option<T> {
    todo!();
}

pub fn max<T: PartialEq + PartialOrd>(node: &Box<BstNode<T>>) -> Option<T> {
    todo!();
}

pub fn size<T: PartialEq + PartialOrd>(node: &Box<BstNode<T>>) -> usize {
    todo!();
}

pub fn height<T: PartialEq + PartialOrd>(node: &Box<BstNode<T>>) -> usize {
    todo!();
}

#[derive(Clone)]
pub struct BstNode<T: PartialEq + PartialOrd> {
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>,
    data: T,
}

impl<T: PartialEq + PartialOrd> BstNode<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_empty() -> BinarySearchTree<usize> {
        BinarySearchTree::new(None)
    }

    fn create_filled() -> BinarySearchTree<usize> {
        BinarySearchTree {
            root: Some(Box::new(BstNode::<usize> {
                data: 50,
                left: Some(Box::new(BstNode::<usize>::new(25))),
                right: Some(Box::new(BstNode::<usize>::new(100))),
            })),
        }
    }

    #[test]
    fn test_insert() {
        let mut bst = create_empty();
        bst.insert(16);
        assert_eq!(bst.root.is_none(), false);
        assert_eq!(bst.root.as_ref().unwrap().data, 16);

        bst.insert(8);
        assert_eq!(bst.root.as_ref().unwrap().left.is_none(), false);
        assert_eq!(bst.root.as_ref().unwrap().left.as_ref().unwrap().data, 8);

        bst.insert(32);
        assert_eq!(bst.root.as_ref().unwrap().right.is_none(), false);
        assert_eq!(bst.root.as_ref().unwrap().right.as_ref().unwrap().data, 32);
    }

    #[test]
    fn test_delete() {
        let mut bst = create_filled();
        bst.delete(25);
        assert_eq!(bst.root.as_ref().unwrap().left.is_none(), true);
        bst.delete(100);
        assert_eq!(bst.root.unwrap().right.is_none(), true);
    }

    #[test]
    fn test_delete_root() {
        // Before remove:
        //    50
        //  /   \
        // 25    100
        // After remove:
        //   20
        //  /  \
        // *    100
        let mut bst = create_filled();
        bst.delete(50);
        assert_eq!(bst.root.unwrap().data, 25);

        // More advanced tree
        // Before remove:
        //        50
        //    /       \
        //   25        100
        //  /  \       / \
        // 17   42    75  150
        // After remove:
        //      42
        //    /    \
        //   25     100
        //  /      /   \
        // 17     75  150
        type Bn8 = BstNode<u8>;
        let mut bst = BinarySearchTree::<u8> {
            root: Some(Box::new(Bn8 {
                data: 50,
                left: Some(Box::new(Bn8 {
                    data: 25,
                    left: Some(Box::new(Bn8::new(17))),
                    right: Some(Box::new(Bn8::new(42))),
                })),
                right: Some(Box::new(Bn8 {
                    data: 100,
                    left: Some(Box::new(Bn8::new(75))),
                    right: Some(Box::new(Bn8::new(150))),
                })),
            })),
        };
        bst.delete(50);
        assert_eq!(bst.root.unwrap().data, 42);
    }

    #[test]
    fn test_contains_found() {
        let mut bst = create_filled();
        let value = bst.contains(25);
        assert!(value);

        let value = bst.contains(50);
        assert!(value);

        let value = bst.contains(100);
        assert!(value);

        bst.insert(17);
        let value = bst.contains(17);
        assert!(value);

        bst.insert(255);
        assert!(bst.contains(255));
    }

    #[test]
    fn test_contains_missing() {
        let bst = create_filled();
        assert_eq!(bst.contains(255), false);
    }

    #[test]
    fn test_min() {
        let mut bst = create_filled();
        let min = bst.min().unwrap();
        assert_eq!(min, 25);

        bst.insert(255);
        let min = bst.min().unwrap();
        assert_eq!(min, 25);

        bst.insert(7);
        let min = bst.min().unwrap();
        assert_eq!(min, 7);
    }

    #[test]
    fn test_min_empty() {
        let bst = create_empty();
        assert_eq!(bst.min().is_none(), true);
    }

    #[test]
    fn test_max() {
        let mut bst = create_filled();
        let max = bst.max().unwrap();
        assert_eq!(max, 100);

        bst.insert(17);
        let max = bst.max().unwrap();
        assert_eq!(max, 100);

        bst.insert(255);
        let max = bst.max().unwrap();
        assert_eq!(max, 255);
    }

    #[test]
    fn test_max_empty() {
        let bst = create_empty();
        assert_eq!(bst.max().is_none(), true);
    }

    #[test]
    fn test_is_empty() {
        let mut bst = create_empty();
        assert_eq!(bst.is_empty(), true);

        bst.insert(255);
        assert_eq!(bst.is_empty(), false);
    }

    #[test]
    fn test_is_empty_filled() {
        let mut bst = create_filled();
        assert_eq!(bst.is_empty(), false);

        bst.root = None;
        assert_eq!(bst.is_empty(), true);
    }

    #[test]
    fn test_size() {
        let mut bst = create_filled();
        let size = bst.size();
        assert_eq!(size, 3);

        bst.insert(255);
        assert_eq!(bst.size(), 4);
    }

    #[test]
    fn test_size_empty() {
        let bst = create_filled();
        let size = bst.size();
        assert_eq!(size, 0);
    }

    #[test]
    fn test_height() {
        let mut bst = create_filled();
        let height = bst.height();
        assert_eq!(height, 2);

        bst.insert(255);
        assert_eq!(bst.height(), 3);
    }

    #[test]
    fn test_height_empty() {
        let bst = create_empty();
        let height = bst.height();
        assert_eq!(height, 0);
    }

    #[test]
    fn test_clear() {
        let mut bst = create_filled();
        bst.clear();
        assert_eq!(bst.root.is_none(), true);
    }
}
