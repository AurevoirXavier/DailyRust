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
        let deal_3 = n % 3 == 0;

        match n % 5 {
            0 => if deal_3 { println!("fizzbuzz") } else { println!("buzz"); }
            _ if deal_3 => println!("fizz"),
            _ => println!("{}", n)
        }

        n += 1;
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

}
