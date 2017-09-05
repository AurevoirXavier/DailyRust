#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Point_ {
    x: i32,
    y: i32,
    z: i32
}

enum Message {
    Hello {
        id: i32
    }
}

fn main() {
    let x = Some(5);

    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:}", y),
        _ => println!("Default case, x = {:?}", x)
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("something else")
    }

    let x = 'c';

    match x {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else")
    }

    let p = Point {
        x: 0,
        y: 7
    };

    let Point {
        x: a,
        y: b
    } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);
    //    println!("{:?}", p);

    match p {
        Point {
            x, y: 0
        } => println!("On the x axis at {}", x),
        Point {
            x: 0,
            y
        } => println!("On the y axis at {}", y),
        Point {
            x, y
        } => println!("On neither axis: ({}, {})", x, y)
    }

    let points = vec![
        Point {
            x: 0,
            y: 0
        },
        Point {
            x: 1,
            y: 5
        },
        Point {
            x: 10,
            y: -3
        }
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point {
            x, y
        }| x * x + y * y)
        .sum();

    println!("{}", sum_of_squares);

    let ((_feet, _inches), Point {
        x: _x,
        y: _y
    }) = ((3, 10), Point {
        x: 3,
        y: -10
    });

    let x = Some(5);

    match x {
        Some(_) => println!("got a Some and I don't care what's inside"),
        None => ()
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    //    let s = Some(String::from("Hello!"));
    //
    //    if let Some(_s) = s {
    //        println!("found a string");
    //    }
    //
    //    println!("{:?}", s);

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point_ {
        x: 0,
        y: 0,
        z: 0
    };

    match origin {
        Point_ {
            x, ..
        } => println!("x is {}", x)
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {} , {}", first, last);
        }
    }

    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => ()
    }

    println!("robot_name is: {:?}", robot_name);

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => ()
    }

    let x = Some(5);

    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case , x = {:?}", x)
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;

    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no")
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3 ... 7 } => {
            println!("Found an id in range: {}", id);
        }
        Message::Hello { id: 10 ... 12 } => {
            println!("Found an id in another range");
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}
