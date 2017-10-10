fn main() {
    println!("{}", diagonal(40, 2));
}

/*
 m
C  = c(n, m) = n!/m!(n-m)!
 n

diagonal(n, p)
-> c(n, p) + c(n-1, p) + ... + c(p, p)
-> n!/p!(n-p)! + (n-1)!/p![(n-1)-p]! + ... + p!/p!(p-p)!
-> n!/p!(n-p)! + (n-1)!/p![(n-1)-p]! + ... + 1

diagonal(7, 2)
-> c(7, 2) + c(6, 2) + ... + c(2, 2)
-> 7!/2!(7-2)! + 6!/2![(6-1)-2]! + ... + 2!/2!(2-2)!
-> 7!/2!(7-2)! + 6!/2![(6-1)-2]! + ... + 1
*/

//fn diagonal(n: u32, p: u32) -> u64 {
//    let mut f = 1f64;
//    let mut d = 1f64;
//
//    for i in 0..(1 + p) {
//        //    f -> (n + 1 - 0) * (n + 1 - 1) * ... * (n + 1 - p)
//        //    f -> (n + 1 - p)! / (n + 1)!
//        f *= n as f64 + 1.0 - i as f64;
//
//        //    d -> 1 * 2 * ... * p
//        //    d -> p!
//        d *= i as f64 + 1.0;
//    }
//
//    //    f / d -> (n + 1 - p)! / p!(n + 1)!
//    (f / d).round() as u64
//}

fn diagonal(n: u32, p: u32) -> u64 {
    let mut a  = p;
    let mut numerator = 1f64;
    let mut fac = 1f64;
    let mut difference = 1f64;
    let mut sum = 1f64;

    while a != n {
        a += 1;
        numerator *= a as f64;
        fac = difference * fac;
        difference += 1f64;
        sum += numerator / fac;
    }

    sum.round() as u64
}
