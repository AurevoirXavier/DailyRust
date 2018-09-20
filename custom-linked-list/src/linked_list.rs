use std::mem;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub enum LinkedList<T> {
    Node {
        val: T,
        next: Box<LinkedList<T>>,
    },
    Empty,
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = LinkedList<T>;

    fn index(&self, index: usize) -> &LinkedList<T> {
        match index {
            0 => self,
            _ => if let LinkedList::Node {
                val: _,
                next,
            } = self { next.index(index - 1) } else { panic!() }
        }
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut LinkedList<T> {
        match index {
            0 => self,
            _ => if let LinkedList::Node {
                val: _,
                next,
            } = self { next.index_mut(index - 1) } else { panic!() }
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> { LinkedList::Empty }

    fn get_self(&mut self) -> LinkedList<T> { mem::replace(self, LinkedList::Empty) }

    fn len(&self) -> usize {
        let mut linked_list = self;
        let mut len = 0;
        loop {
            if let LinkedList::Node {
                val: _,
                next,
            } = linked_list {
                linked_list = next;
                len += 1;
            } else { return len; }
        }
    }

    pub fn insert(&mut self, index: usize, ins_val: T) {
        if index == 0 {
            let ins_next = self.get_self();
            mem::replace(self, LinkedList::Node {
                val: ins_val,
                next: Box::new(ins_next),
            });
        } else {
            if let LinkedList::Node {
                val: _,
                ref mut next,
            } = self[index - 1] {
                let ins_next = mem::replace(next, Box::new(LinkedList::Empty));
                *next = Box::new(LinkedList::Node {
                    val: ins_val,
                    next: ins_next,
                });
            }
        }
    }

    pub fn remove(&mut self, index: usize) {
        let len = self.len();
        if index == len { self[len - 1] = LinkedList::Empty; } else {
            let prev = &mut self[index - 1];
            let ins_next = prev[2].get_self();
            if let LinkedList::Node {
                val: _,
                ref mut next,
            } = prev { *next = Box::new(ins_next); }
        }
    }
}