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
pub mod deque_array_list;
pub mod deque_linked_list;

pub use deque_array_list::DequeArrayList;
pub use deque_linked_list::DequeLinkedList;

pub trait Deque<T> {
    fn push_front(&mut self, element: Node<T>);
    fn push_back(&mut self, element: Node<T>);
    fn pop_front(&mut self) -> Option<Node<T>>;
    fn pop_back(&mut self) -> Option<Node<T>>;
    fn front(&self) -> Option<&Node<T>>;
    fn back(&self) -> Option<&Node<T>>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

pub struct Node<T> {
    pub data: Option<T>,
}

impl<T> Node<T> {
    fn new() -> Self {
        Self { data: None }
    }

    fn from(data: T) -> Self {
        Self { data: Some(data) }
    }
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self::new()
    }
}
