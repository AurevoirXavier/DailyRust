fn a_fn_that_immutably_borrows(a: &i32) {
    println!("a is {}", a);
}

fn a_fn_that_mutably_borrows(b: &mut i32) {
    *b += 1;
}

fn demo(r: &RefCell<i32>) {
    a_fn_that_immutably_borrows(&r.borrow());
    a_fn_that_mutably_borrows(&mut r.borrow_mut());
    a_fn_that_immutably_borrows(&r.borrow());
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    demo(&data);

    let value = Rc::new(RefCell::new(5));

    let a = Cons(value.clone(), Rc::new(Nil));

    let shared_list = Rc::new(a);

    let b = Cons(Rc::new(RefCell::new(6)), shared_list.clone());
    let c = Cons(Rc::new(RefCell::new(10)), shared_list.clone());

    println!("shared_list before = {:?}", shared_list);
    println!("b before  = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("shared_list after = {:?}", shared_list);
    println!("b after  = {:?}", b);
    println!("c after = {:?}", c);
}
