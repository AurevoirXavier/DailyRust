#[allow(unreachable_code)]
fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }

        println!("unreachable");
    }

    println!("Exited the outer loop");

    let mut cnt = 0;
    let result = loop {
        cnt += 1;

        if cnt == 10 { break cnt * 2; }
    };

    println!("{}", result);

    let mut n = 1;

    while n < 101 {
        match n {}
        n += 1;
    }
}
