struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn square(&self, k: f32) -> (Point, f32, f32) {
        let Point { x: x, y: y } = self;
        let (x, y) = (x + k, y + y);

        (
            Point { x, y },
            x,
            y
        )
    }
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 }
        } = self;

        (x1 - x2).abs() * (y1 - y2).abs()
    }
}

fn main() {
    let point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let Point { x: my_x, y: my_y } = point;

    let rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: Point { x: 0.5, y: 0.7 },
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("{}", rectangle.rect_area());

    println!("{:?}", point.square(0.7));
}
