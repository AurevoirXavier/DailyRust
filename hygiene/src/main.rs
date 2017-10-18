fn main() {
//    macro_rules! using_a {
//        ($e:expr) => {
//             {
//                let a = 42;
//                $e
//            }
//        }
//    }

//    let four = using_a!(a / 10);


    macro_rules! using_a {
        ($a:ident, $e:expr) => {
            {
                let $a = 42f64;
                $e
            }
        }
    }

    let x = using_a!(a, a / 10f64);

    println!("{}", x);
}
