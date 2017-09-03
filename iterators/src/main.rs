struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, [2, 3, 4]);

    let mut counter = Counter::new();

    let x = counter.next();
    println!("{:?}", x);

    let x = counter.next();
    println!("{:?}", x);

    let x = counter.next();
    println!("{:?}", x);

    let x = counter.next();
    println!("{:?}", x);

    let x = counter.next();
    println!("{:?}", x);

    let x = counter.next();
    println!("{:?}", x);

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
