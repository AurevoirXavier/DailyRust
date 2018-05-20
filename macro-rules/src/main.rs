#![feature(trace_macros)]
#![feature(log_syntax)]

macro_rules! four {
    () => {1 + 3};
}

macro_rules! gibberish {
    (4 fn ['spang "whammo"] @_@) => {1};
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

macro_rules! capture_then_match_tokens {
    ($e:expr) => {match_tokens!($e)};
}

macro_rules! match_tokens {
    ($a:tt + $b:tt) => {"got an addition"};
    (($i:ident)) => {"got an identifier"};
    ($($other:tt)*) => {"got something else"};
}

macro_rules! capture_then_what_is_1 {
    (#[$m:meta]) => {what_is_1!(#[$m])};
}

macro_rules! what_is_1 {
    (#[no_mangle]) => {"no_mangle attribute"};
    (#[inline]) => {"inline attribute"};
    ($($tts:tt)*) => {concat!("something else (", stringify!($($tts)*), ")")};
}

macro_rules! using_a {
    ($a:ident, $e:expr) => {
        {
            let mut $a = 42;

            $a += 8;
            $e
        }
    }
}

macro_rules! what_is_2 {
    (self) => {"the keyword `self`"};
    ($i:ident) => {concat!("the identifier `", stringify!($i), "`")};
}

macro_rules! call_with_ident {
    ($c:ident($i:ident)) => {$c!($i)};
}

macro_rules! double_method {
    ($self:ident, $body:expr) => {
        fn double(mut $self) -> Dummy { $body }
    };
}

struct Dummy(i32);

impl Dummy {
    double_method! {self, {
        self.0 *= 2;
        self
    }}
}

macro_rules! each_tt {
    () => {};
    ($_tt:tt $($rest:tt)*) => {each_tt!($($rest)*);};
}

macro_rules! sing {
    () => {};
    ($tt:tt $($rest:tt)*) => {log_syntax!($tt); sing!($($rest)*);};
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

    println!(
        "{}\n{}\n{}",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5)
    );
    println!(
        "{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5)
    );

    println!(
        "{}\n{}\n{}\n{}",
        what_is_1!(#[no_mangle]),
        what_is_1!(#[inline]),
        capture_then_what_is_1!(#[no_mangle]),
        capture_then_what_is_1!(#[inline]),
    );

    let x = using_a!(a, a / 10);
    println!("{}", x);

    println!("{}", what_is_2!(self));
    println!("{}", call_with_ident!(what_is_2(self)));

    let x = Dummy(1).double();
    println!("{}", x.0);

//    each_tt!(foo bar baz quux);
    trace_macros!(true);
    each_tt!(spim wak plee whum);
    trace_macros!(false);
//    each_tt!(trom, qlip, winp, xod);

    sing! {
        ^ < @ < . @ *
        '\x08' '{' '"' _ # ' '
        - @ '$' && / _ %
        ! ( '\t' @ | = >
        ; '\x08' '\'' + '$' ? '\x7f'
        , # '"' ~ | ) '\x07'
    }

    let fib = {
        use std::ops::Index;

        #[derive(Debug)]
        struct Recurrence {
            mem: [u64; 2],
            pos: usize,
        }

        struct IndexOffset<'a> {
            slice: &'a [u64; 2],
            offset: usize,
        }

        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = u64;

            #[inline(always)]
            fn index<'b>(&'b self, index: usize) -> &'b u64 {
                use std::num::Wrapping;

                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let windows = Wrapping(2);
                let real_index = index - offset + windows;

                &self.slice[real_index.0]
            }
        }

        impl Iterator for Recurrence {
            type Item = u64;

            #[inline]
            fn next(&mut self) -> Option<u64> {
                if self.pos < 2 {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;

                    Some(next_val)
                } else {
                    let next_val = {
                        let n = self.pos;
                        let a = IndexOffset { slice: &self.mem, offset: n };

                        (a[n - 1] + a[n - 2])
                    };

                    {
                        use std::mem::swap;

                        let mut swap_tmp = next_val;
                        for i in (0..2).rev() {
                            swap(&mut swap_tmp, &mut self.mem[i]);
                        }
                    }

                    self.pos += 1;

                    Some(next_val)
                }
            }
        }

        Recurrence { mem: [0, 1], pos: 0 }
    };

//    for e in fib.take(10) { println!("{}", e); }

    let mut x = fib;
    for _ in 0..10 { x.next(); }
    println!("{:?}", x);
}
