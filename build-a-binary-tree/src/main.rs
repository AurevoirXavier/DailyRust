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
            *node = Some(Box::new(Node {
                val: insert_val,
                l: None,
                r: None,
            }));
        }
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut queue = VecDeque::from(vec![Some(self)]);
        let mut tree = vec![];

        while let Some(node) = queue.pop_front() {
            if let Some(Node { val, l, r }) = node {
                tree.push(val.to_string());

                if l.is_none() && r.is_none() { continue; }
                if let Some(node) = l { queue.push_back(Some(node)); } else { queue.push_back(None); }
                if let Some(node) = r { queue.push_back(Some(node)); } else { queue.push_back(None); }
            } else { tree.push("".to_string()); }
        }

        let mut str = String::new();
        let mut edge = 1u32;
        let mut curr = 0u32;

        for node in tree.into_iter() {
            match node.as_str() {
                "" => str.push('_'),
                node => str.push_str(node)
            }

            curr += 1;

            if curr != edge { str.push(' '); } else {
                str.push('\n');

                curr = 0;
                edge *= 2;
            }
        }

        write!(f, "{}", str)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Tree<T: Ord + fmt::Display> {
    Node {
        val: T,
        l: Box<Tree<T>>,
        r: Box<Tree<T>>,
    },
    Empty,
}

impl<T: Ord + fmt::Display> Tree<T> {
    fn new() -> Tree<T> { Tree::Empty }

    fn insert(&mut self, nv: T) {
        if let &mut Tree::Node { ref val, ref mut l, ref mut r } = self {
            match nv.cmp(val) {
                Ordering::Less => l.insert(nv),
                Ordering::Greater => r.insert(nv),
                _ => return
            }
        } else {
            *self = Tree::Node {
                val: nv,
                l: Box::new(Tree::Empty),
                r: Box::new(Tree::Empty),
            }
        }
    }

    fn is_empty(&self) -> bool { if let &Tree::Node { .. } = self { false } else { true } }

    fn find(&self, fv: T) -> bool {
        if let &Tree::Node { ref val, ref l, ref r } = self {
            match fv.cmp(val) {
                Ordering::Less => r.find(fv),
                Ordering::Greater => l.find(fv),
                _ => true
            }
        } else { false }
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
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

    println!("Struct:\n{}", x);

    let mut x = Tree::new();

    assert!(x.is_empty());

    x.insert("m");
    x.insert("z");
    x.insert("a");
    x.insert("b");

    assert_eq!(
        x,
        Tree::Node {
            val: "m",
            l: Box::new(Tree::Node {
                val: "a",
                l: Box::new(Tree::Empty),
                r: Box::new(Tree::Node {
                    val: "b",
                    l: Box::new(Tree::Empty),
                    r: Box::new(Tree::Empty),
                }),
            }),
            r: Box::new(Tree::Node {
                val: "z",
                l: Box::new(Tree::Empty),
                r: Box::new(Tree::Empty),
            }),
        }
    );

    x.insert("c");
    x.insert("v");

    println!("Enum:\n{}", x);

//    let a = vec![1, 2, 3];
//    match a {
//        val => println!("{:?}", std::mem::size_of_val(&val))
//    }
//
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
