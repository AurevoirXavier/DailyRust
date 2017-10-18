fn main() {
    macro_rules! four {
        () => {1 + 3};
    }

    let x = four!();

    println!("{}", x);

    macro_rules! gibberish {
        (4 fn ['spang "whammo"] @_@) => {...};
    }

    macro_rules! multiply_add {
        ($a:expr, $b:expr, $c:expr) => {$a * ($b + $c)};
    }

    let x = multiply_add!(2, 2, 3);

    println!("{}", x);
}

