// macro_rules! using_a {
//     ($e:expr) => {
//          {
//             let a = 42;
//             $e
//         }
//     }
// }

macro_rules! using_a {
    ($a:ident, $e:expr) => {
        {
            let $a = 42f64;
            $e
        }
    }
}

fn main() {
//    let four = using_a!(a / 10);

    let x = using_a!(a, a / 10f64);

    println!("{}", x);
}
