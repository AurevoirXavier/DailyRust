use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut data = stdin.lock().lines();

    let (w, h, n);
    {
        let mut data = next_datum(&mut data);

        w = data[0];
        h = data[1];
        n = data[2];
    }

    let mut vals = vec![];
    for _ in 0..h { vals.push(next_datum(&mut data)); }

    let mut squares = vec![];
    for _ in 0..n { squares.push(next_datum(&mut data)); }

    println!(
        "{:?}",
        squares.iter()
            .filter(|v| 1 >= v[1] && 1 <= v[3])
            .collect::<Vec<_>>()
    );
//    let mut sum = 0;
//    for r in 1..h + 1 {
//        let squares: Vec<_> = squares.iter()
//            .filter(|v| r >= v[1] && r <= v[3])
//            .collect();
//
//        for c in 1..w + 1 {
//            if squares.iter().any(|v| c >= v[0] && c <= v[2]) {
//                sum += vals[r - 1][c - 1];
//
//                break;
//            }
//        }
//    }

//    println!("{}", sum);
}

fn next_datum(l: &mut io::Lines<io::StdinLock>) -> Vec<usize> {
    l.next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}