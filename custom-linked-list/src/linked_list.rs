use std::mem;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub enum LinkedListEnum<T> {
    Node {
        val: T,
        next: Box<LinkedListEnum<T>>,
    },
    Empty,
}

impl<T> Index<usize> for LinkedListEnum<T> {
    type Output = LinkedListEnum<T>;

    fn index(&self, index: usize) -> &LinkedListEnum<T> {
        match index {
            0 => self,
            _ => if let LinkedListEnum::Node {
                val: _,
                next,
            } = self { next.index(index - 1) } else { panic!() }
        }
    }
}

impl<T> IndexMut<usize> for LinkedListEnum<T> {
    fn index_mut(&mut self, index: usize) -> &mut LinkedListEnum<T> {
        match index {
            0 => self,
            _ => if let LinkedListEnum::Node {
                val: _,
                next,
            } = self { next.index_mut(index - 1) } else { panic!() }
        }
    }
}

impl<T> LinkedListEnum<T> {
    pub fn new() -> LinkedListEnum<T> { LinkedListEnum::Empty }

    pub fn insert(&mut self, index: usize, ins_val: T) {
        if index == 0 {
            let ins_next = mem::replace(self, LinkedListEnum::Empty);
            mem::replace(self, LinkedListEnum::Node {
                val: ins_val,
                next: Box::new(ins_next),
            });
        } else {
            if let LinkedListEnum::Node {
                val: _,
                ref mut next,
            } = self[index - 1] {
                *next = Box::new(LinkedListEnum::Node {
                    val: ins_val,
                    next: mem::replace(next, Box::new(LinkedListEnum::Empty)),
                });
            }
        }
    }

    pub fn remove(&mut self, index: usize) {
        let prev = &mut self[index - 1];
        let ins_next = mem::replace(&mut prev[2], LinkedListEnum::Empty);
        if let LinkedListEnum::Node {
            val: _,
            ref mut next,
        } = prev { *next = Box::new(ins_next); }
    }
}

#[derive(Debug)]
pub struct LinkedListStruct<T> {
    val: Option<T>,
    next: Option<Box<LinkedListStruct<T>>>,
}

impl<T> Index<usize> for LinkedListStruct<T> {
    type Output = LinkedListStruct<T>;

    fn index(&self, index: usize) -> &LinkedListStruct<T> {
        let mut find = self;
        for _ in 0..index {
            if let Some(ref next) = find.next {
                find = next;
            } else { panic!() }
        }

        find
    }
}

impl<T> IndexMut<usize> for LinkedListStruct<T> {
    fn index_mut(&mut self, index: usize) -> &mut LinkedListStruct<T> {
        let mut find = self;
        for _ in 0..index {
            if let Some(ref mut next) = find.next {
                find = next;
            } else { panic!() }
        }

        find
    }
}

impl<T> LinkedListStruct<T> {
    pub fn new() -> LinkedListStruct<T> {
        LinkedListStruct {
            val: None,
            next: None,
        }
    }

    pub fn insert(&mut self, index: usize, ins_val: T) {
        if index == 0 { self.val = Some(ins_val); } else {
            let prev = &mut self[index - 1];
            prev.next = Some(Box::new(LinkedListStruct {
                val: Some(ins_val),
                next: mem::replace(&mut prev.next, None),
            }));
        }
    }

    pub fn remove(&mut self, index: usize) {
        if index == 0 {
            if let Some(ref mut next) = self.next {
                self.val = mem::replace(&mut next.val, None);
                self.next = mem::replace(&mut next.next, None);
            } else { self.val = None; }
        } else {
            let prev = &mut self[index - 1];
            prev.next = mem::replace(&mut prev[1].next, None);
        }
    }
}