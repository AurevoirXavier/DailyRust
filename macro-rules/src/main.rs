macro_rules! four {
    () => {1 + 3};
}

macro_rules! gibberish {
    (4 fn ['spang "whammo"] @_@) => { 1 };
}

macro_rules! times_five {
    ($e:expr) => {5 * $e};
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {$a * ($b + $c)};
}

macro_rules! vec_strs {
    ($(($a:expr, $b:expr)),*) => {{
        let mut v = Vec::new();

        $(v.push(format!("{}", $b));)*

        v
    }};

    ($($element:expr),*) => {{
        let mut v = Vec::new();

        $(v.push(format!("{}", $element));)*

        v
    }};
}

macro_rules! capture_expr_then_stringify {
    ($e:expr) => {stringify!($e)};
}

fn main() {
    let x = four!();
    println!("{}", x);

    let x = gibberish!(4 fn ['spang "whammo"] @_@);
    println!("{}", x);

    let x = times_five!(2);
    println!("{}", x);

    let x = multiply_add!(2, 2, 3);
    println!("{}", x);

    let x = vec_strs!(1, 2, 3, 4);
    println!("{:?}", x);

    let x = vec_strs!((1, 2), (3, 4));
    println!("{:?}", x);

    println!("{:?}", stringify!(dummy(2 * (1 + (3)))));
    println!("{:?}", capture_expr_then_stringify!(dummy(2 * (1 + (3)))));
}
