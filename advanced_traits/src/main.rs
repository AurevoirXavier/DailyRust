trait GGraph<Node, Edge> {}

trait AGraph {
    type Node;
    type Edge;
}

trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();

        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

struct Millimeters(u32);

struct Meters(u32);

struct Baz;

struct Point_ {
    x: i32,
    y: i32
}

struct Wrapper(Vec<String>);

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Foo for Baz {
    fn f(&self) {
        println!("Baz's impl of Foo");
    }
}

impl Bar for Baz {
    fn f(&self) {
        println!("Baz's impl of Bar");
    }
}

impl Baz {
    fn f(&self) {
        println!("Baz's impl");
    }
}

impl fmt::Display for Point_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point_ {}

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

fn _distance_g<N, E, G: GGraph<N, E>>(_graph: &G, _start: &N, _end: &N) {}

fn _distance_a<G: AGraph>(_graph: &G, _start: &G::Node, _end: &G::Node) {}

fn _distance_t<N, E>(_graph: &GGraph<N, E>, _start: &N, _end: &N) {}

fn _traverse(_graph: &AGraph<Node=usize, Edge=(usize, usize)>) {}


fn main() {
    assert_eq!(Point {
        x: 1,
        y: 0
    } + Point {
        x: 2,
        y: 3
    }, Point {
        x: 3,
        y: 3
    });

    let b = Baz;
    b.f();
    <Baz as Foo>::f(&b);
    <Baz as Bar>::f(&b);

    let point = Point_ {
        x: 1,
        y: 3
    };

    point.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("w = {}", w);
}
