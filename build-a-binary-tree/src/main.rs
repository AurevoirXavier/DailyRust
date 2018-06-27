#[derive(Debug)]
#[derive(PartialEq)]
struct Node<T: Ord> {
    v: T,
    l: Option<Box<Node<T>>>,
    r: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn insert(&mut self, nv: T) {
        if self.v == nv { return; }

        let target_node = if nv < self.v { &mut self.l } else { &mut self.r };

        if let &mut Some(ref mut sub_node) = target_node { sub_node.insert(nv); } else {
            let new_node = Node {
                v: nv,
                l: None,
                r: None,
            };
            let boxed_node = Some(Box::new(new_node));

            *target_node = boxed_node;
        }
    }
}

use std::cmp::Ordering;

#[derive(Debug)]
#[derive(PartialEq)]
enum Tree<T: Ord> {
    Leaf {
        v: T,
        l: Box<Tree<T>>,
        r: Box<Tree<T>>,
    },
    Empty,
}

impl<T: Ord> Tree<T> {
    fn new() -> Tree<T> { Tree::Empty }

    fn insert(&mut self, nv: T) {
        if let &mut Tree::Leaf {
            ref v,
            ref mut l,
            ref mut r
        } = self {
            match nv.cmp(v) {
                Ordering::Less => l.insert(nv),
                Ordering::Greater => r.insert(nv),
                _ => return
            }
        } else {
            *self = Tree::Leaf {
                v: nv,
                l: Box::new(Tree::Empty),
                r: Box::new(Tree::Empty),
            }
        }
    }

    fn is_empty(&self) -> bool { if let &Tree::Leaf { .. } = self { false } else { true } }

    fn find(&self, fv: T) -> bool {
        if let &Tree::Leaf {
            ref v,
            ref l,
            ref r
        } = self {
            match fv.cmp(v) {
                Ordering::Less => r.find(fv),
                Ordering::Greater => l.find(fv),
                _ => true
            }
        } else { false }
    }
}

fn main() {
    let mut x = Node { v: "m", l: None, r: None };

    x.insert("z");
    x.insert("a");
    x.insert("b");

    assert_eq!(
        x,
        Node {
            v: "m",
            l: Some(Box::new(Node {
                v: "a",
                l: None,
                r: Some(Box::new(Node {
                    v: "b",
                    l: None,
                    r: None,
                })),
            })),
            r: Some(Box::new(Node {
                v: "z",
                l: None,
                r: None,
            })),
        }
    );

    let mut x = Tree::new();

    assert!(x.is_empty());

    x.insert("m");
    x.insert("z");
    x.insert("a");
    x.insert("b");

    assert_eq!(
        x,
        Tree::Leaf {
            v: "m",
            l: Box::new(Tree::Leaf {
                v: "a",
                l: Box::new(Tree::Empty),
                r: Box::new(Tree::Leaf {
                    v: "b",
                    l: Box::new(Tree::Empty),
                    r: Box::new(Tree::Empty),
                }),
            }),
            r: Box::new(Tree::Leaf {
                v: "z",
                l: Box::new(Tree::Empty),
                r: Box::new(Tree::Empty),
            }),
        }
    );

//    let a = vec![1, 2, 3];
//    match a {
//        val => println!("{:?}", std::mem::size_of_val(&val))
//    }

//    let a = Some("Hello!".to_string());
//
//    match a {
//        Some(s) => {
//            // s is a String here, and therefore
//            // is owned by the match
//        }
//        None => {}
//    }
//
//    println!("{:?}", a);
}
