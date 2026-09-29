// Phase 5 — Standard Collections Interfaces with Runtime Assertions
// Provides checked and typed wrappers for collections.

use std::collections::VecDeque;

pub trait Collection {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn clear(&mut self);
}

pub struct CheckedVector<T> {
    data: Vec<T>,
}

impl<T> CheckedVector<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> Default for CheckedVector<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CheckedDeque<T> {
    data: VecDeque<T>,
}

impl<T> CheckedDeque<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn push_back(&mut self, item: T) {
        self.data.push_back(item);
    }

    pub fn push_front(&mut self, item: T) {
        self.data.push_front(item);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> Default for CheckedDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}
