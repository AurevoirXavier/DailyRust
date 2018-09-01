#![feature(test)]

extern crate test;
extern crate time;

fn new_vec(len: usize) -> Vec<usize> {
    let mut v = vec![];
    for n in (0..len).rev() {
        v.push(n);
    }

    v.sort();

    v
}

#[cfg(test)]
mod tests {
    use super::new_vec;
    use test::{Bencher, black_box};
    use time::PreciseTime;

    #[test]
    fn time_new_vec() {
        let start = PreciseTime::now();
        new_vec(1000000);
        let end = PreciseTime::now();

        println!("{}", start.to(end));
    }

    #[bench]
    fn bench_new_vec(b: &mut Bencher) {
        b.iter(|| new_vec(1000000));
    }

    #[bench]
    fn bench_square_10_times(b: &mut Bencher) {
        b.iter(|| {
            let n = black_box(10);
            (0..n).fold(0, |acc, _| acc ^ 2)
        });
    }
}