macro_rules! four {
    () => {1 + 3};
}

macro_rules! gibberish {
        (4 fn ['spang "whammo"] @_@) => {...};
    }

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {$a * ($b + $c)};
}

fn main() {
    let x = four!();

    println!("{}", x);

    let x = multiply_add!(2, 2, 3);

    println!("{}", x);
}

