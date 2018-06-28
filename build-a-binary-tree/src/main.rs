use std::fmt;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug)]
#[derive(PartialEq)]
struct Node<T: Ord + fmt::Display> {
    val: T,
    l: Option<Box<Node<T>>>,
    r: Option<Box<Node<T>>>,
}

impl<T: Ord + fmt::Display> Node<T> {
    pub fn insert(&mut self, insert_val: T) {
        if self.val == insert_val { return; }

        let node = if insert_val < self.val { &mut self.l } else { &mut self.r };

        if let &mut Some(ref mut sub_node) = node { sub_node.insert(insert_val); } else {
            let new_node = Node {
                val: insert_val,
                l: None,
                r: None,
            };
            let boxed_node = Some(Box::new(new_node));

            *node = boxed_node;
        }
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut queue = VecDeque::from(vec![Some(self)]);
        let mut tree = String::new();
        let mut i: u32 = 1;
        let mut j: u32 = 1;
        let mut k: u32 = 0;

        while let Some(node) = queue.pop_front() {
            if let Some(Node { val, l, r }) = node {
                k = 0;

                tree.push_str(&val.to_string());

                if i == j {
                    tree.push('\n');

                    i = 1;
                    j *= 2;
                } else {
                    tree.push(' ');

                    i += 1;
                }

                if let Some(node) = l { queue.push_back(Some(node)); } else { queue.push_back(None); }
                if let Some(node) = r { queue.push_back(Some(node)); } else { queue.push_back(None); }
            } else {
                if k == j { break; }

                k += 1;

                tree.push('_');

                if i == j {
                    tree.push('\n');

                    i = 1;
                    j *= 2;
                } else {
                    tree.push(' ');

                    i += 1;
                }

                queue.push_back(None);
                queue.push_back(None);
            }
        }

        let tree: Vec<&str> = tree.split('\n').collect();

        write!(f, "{}", tree[..tree.len() - 1].join("\n"))
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Tree<T: Ord> {
    Leaf {
        val: T,
        l: Box<Tree<T>>,
        r: Box<Tree<T>>,
    },
    Empty,
}

impl<T: Ord> Tree<T> {
    fn new() -> Tree<T> { Tree::Empty }

    fn insert(&mut self, nv: T) {
        if let &mut Tree::Leaf { ref val, ref mut l, ref mut r } = self {
            match nv.cmp(val) {
                Ordering::Less => l.insert(nv),
                Ordering::Greater => r.insert(nv),
                _ => return
            }
        } else {
            *self = Tree::Leaf {
                val: nv,
                l: Box::new(Tree::Empty),
                r: Box::new(Tree::Empty),
            }
        }
    }

    fn is_empty(&self) -> bool { if let &Tree::Leaf { .. } = self { false } else { true } }

    fn find(&self, fv: T) -> bool {
        if let &Tree::Leaf { ref val, ref l, ref r } = self {
            match fv.cmp(val) {
                Ordering::Less => r.find(fv),
                Ordering::Greater => l.find(fv),
                _ => true
            }
        } else { false }
    }
}

fn main() {
    let mut x = Node { val: "m", l: None, r: None };

    x.insert("z");
    x.insert("a");
    x.insert("b");

    assert_eq!(
        x,
        Node {
            val: "m",
            l: Some(Box::new(Node {
                val: "a",
                l: None,
                r: Some(Box::new(Node {
                    val: "b",
                    l: None,
                    r: None,
                })),
            })),
            r: Some(Box::new(Node {
                val: "z",
                l: None,
                r: None,
            })),
        }
    );

    x.insert("c");
    x.insert("v");

    println!("{}", x);

    let mut x = Tree::new();

    assert!(x.is_empty());

    x.insert("m");
    x.insert("z");
    x.insert("a");
    x.insert("b");

    assert_eq!(
        x,
        Tree::Leaf {
            val: "m",
            l: Box::new(Tree::Leaf {
                val: "a",
                l: Box::new(Tree::Empty),
                r: Box::new(Tree::Leaf {
                    val: "b",
                    l: Box::new(Tree::Empty),
                    r: Box::new(Tree::Empty),
                }),
            }),
            r: Box::new(Tree::Leaf {
                val: "z",
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
