fn main() {
    println!("{}", add(&[4, -3, -2]));
}

fn add(args: &[i64]) -> i64 {
    //    let mut i = 0;
    //
    //    args.iter().map(|x| {
    //        i += 1;
    //
    //        x * i
    //    }).sum()

    args.into_iter().zip(1..).map(|(n, i)| n * i).sum()
}
