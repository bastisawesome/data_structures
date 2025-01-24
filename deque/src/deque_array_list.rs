use std::{array, ops::Sub};

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
use super::{Deque, Node};

pub struct DequeArrayList<T> {
    list: Vec<Node<T>>,
    first: usize,
    last: usize,
}

impl<T> DequeArrayList<T> {
    fn new() -> Self {
        Self {
            list: vec![],
            first: 0,
            last: 0,
        }
    }

    fn from_vec(v: Vec<Node<T>>) -> Self {
        let last = v.len() - 1;

        Self {
            list: v,
            first: 0,
            last,
        }
    }
}

impl<T> Deque<T> for DequeArrayList<T> {
    fn push_front(&mut self, element: Node<T>) {
        self.list.insert(self.first, element);
        self.last = self.list.len() - 1;
    }

    fn push_back(&mut self, element: Node<T>) {
        self.list.push(element);
        self.last = self.list.len() - 1;
    }

    fn pop_front(&mut self) -> Option<Node<T>> {
        if self.is_empty() {
            return None;
        }

        let elem = self.list.remove(self.first);
        self.last = self.list.len().checked_sub(1).unwrap_or(0);

        Some(elem)
    }

    fn pop_back(&mut self) -> Option<Node<T>> {
        if self.is_empty() {
            return None;
        }

        let elem = self.list.remove(self.last);
        self.last = self.list.len().checked_sub(1).unwrap_or(0);

        Some(elem)
    }

    fn front(&self) -> Option<&Node<T>> {
        match self.is_empty() {
            true => return None,
            false => return Some(&self.list[self.first]),
        }
    }

    fn back(&self) -> Option<&Node<T>> {
        match self.is_empty() {
            true => return None,
            false => return Some(&self.list[self.last]),
        }
    }

    fn is_empty(&self) -> bool {
        return self.list.len() == 0;
    }

    fn len(&self) -> usize {
        return self.list.len();
    }
}

#[cfg(test)]
mod test {
    use super::DequeArrayList;
    use crate::{Deque, Node};

    type UNode = Node<usize>;
    type UDeque = DequeArrayList<usize>;

    #[test]
    fn test_push_front_empty() {
        let mut deque = UDeque::new();
        deque.push_front(UNode::from(255));
        assert_eq!(deque.back().unwrap().data, Some(255));
        assert_eq!(deque.front().unwrap().data, Some(255));
    }

    #[test]
    fn test_push_front_filled() {
        let mut deque = UDeque::from_vec(vec![
            UNode::from(1),
            UNode::from(2),
            UNode::from(3),
        ]);
        deque.push_front(UNode::from(255));
        assert_eq!(deque.front().unwrap().data, Some(255));
        assert_eq!(deque.back().unwrap().data, Some(3))
    }

    #[test]
    fn test_push_back_empty() {
        let mut deque = UDeque::new();
        deque.push_back(UNode::from(255));
        assert_eq!(deque.back().unwrap().data, Some(255));
        assert_eq!(deque.front().unwrap().data, Some(255));
    }

    #[test]
    fn test_push_back_filled() {
        let mut deque = UDeque::from_vec(vec![
            UNode::from(1),
            UNode::from(2),
            UNode::from(3),
        ]);
        deque.push_back(UNode::from(255));
        assert_eq!(deque.front().unwrap().data, Some(1));
        assert_eq!(deque.back().unwrap().data, Some(255))
    }

    #[test]
    fn test_pop_front_empty() {
        let mut deque = UDeque::new();
        let elem = deque.pop_front();
        assert_eq!(elem.is_none(), true);
        assert_eq!(deque.front().is_none(), true);
        assert_eq!(deque.back().is_none(), true);
    }

    #[test]
    fn test_pop_front_one_element() {
        let mut deque = UDeque::from_vec(vec![UNode::from(1)]);
        let elem = deque.pop_front();
        assert_eq!(elem.unwrap().data, Some(1));
        assert_eq!(deque.front().is_none(), true);
        assert_eq!(deque.back().is_none(), true);
    }

    #[test]
    fn test_pop_front_filled() {
        let mut deque = UDeque::from_vec(vec![
            UNode::from(1),
            UNode::from(2),
            UNode::from(3),
        ]);
        let elem = deque.pop_front();
        assert_eq!(elem.unwrap().data, Some(1));
        assert_eq!(deque.front().unwrap().data, Some(2));
        assert_eq!(deque.back().unwrap().data, Some(3));
    }

    #[test]
    fn test_pop_back_empty() {
        let mut deque = UDeque::new();
        let elem = deque.pop_back();
        assert_eq!(elem.is_none(), true);
        assert_eq!(deque.front().is_none(), true);
        assert_eq!(deque.back().is_none(), true);
    }

    #[test]
    fn test_pop_back_one_element() {
        let mut deque = UDeque::from_vec(vec![UNode::from(1)]);
        let elem = deque.pop_back();
        assert_eq!(elem.unwrap().data, Some(1));
        assert_eq!(deque.front().is_none(), true);
        assert_eq!(deque.back().is_none(), true);
    }

    #[test]
    fn test_pop_back_filled() {
        let mut deque = UDeque::from_vec(vec![
            UNode::from(1),
            UNode::from(2),
            UNode::from(3),
        ]);
        let elem = deque.pop_back();
        assert_eq!(elem.unwrap().data, Some(3));
        assert_eq!(deque.front().unwrap().data, Some(1));
        assert_eq!(deque.back().unwrap().data, Some(2));
    }

    #[test]
    fn test_front_empty() {
        let deque = UDeque::new();
        assert_eq!(deque.front().is_none(), true);
    }

    #[test]
    fn test_front_filled() {
        let deque = UDeque::from_vec(vec![
            UNode::from(1),
            UNode::from(2),
            UNode::from(3),
        ]);
        assert_eq!(deque.front().unwrap().data, Some(1));
    }

    #[test]
    fn test_back_empty() {
        let deque = UDeque::new();
        assert_eq!(deque.back().is_none(), true);
    }

    #[test]
    fn test_back_filled() {
        let deque = UDeque::from_vec(vec![
            UNode::from(1),
            UNode::from(2),
            UNode::from(3),
        ]);
        assert_eq!(deque.back().unwrap().data, Some(3));
    }

    #[test]
    fn test_empty_empty() {
        let deque = UDeque::new();
        assert_eq!(deque.is_empty(), true);
    }

    #[test]
    fn test_empty_filled() {
        let deque =
            UDeque::from_vec(vec![Node::from(1), Node::from(2), Node::from(3)]);
        assert_eq!(deque.is_empty(), false);
    }

    #[test]
    fn test_size_empty() {
        let deque = UDeque::new();
        assert_eq!(deque.len(), 0);
    }

    #[test]
    fn test_size_filled() {
        let deque = UDeque::from_vec(vec![
            UNode::from(1),
            UNode::from(2),
            UNode::from(3),
        ]);
        assert_eq!(deque.len(), 3);
    }
}
