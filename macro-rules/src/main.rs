macro_rules! four {
    () => {1 + 3};
}

macro_rules! gibberish {
        (4 fn ['spang "whammo"] @_@) => { 1 };
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {$a * ($b + $c)};
}

marco_rules! times_five {
    ($e:expr) => {t * $e};
}

fn main() {
    let x = four!();
    println!("{}", x);

    let x = gibberish!(4 fn ['spang "whammo"] @_@);
    println!("{}", x);

    let x = multiply_add!(2, 2, 3);
    println!("{}", x);

    let x = times_five!();
    println!("{}", x);
}

