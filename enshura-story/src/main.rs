// https://paiza.jp/poh/enshura

use std::io::{self, BufRead};

fn main() {
//    test_1();
//    test_2();
//    test_3();
//    test_4();
}

fn test_1() {
    let mut ciphertext = String::new();
    io::stdin().read_line(&mut ciphertext);

    println!(
        "{}",
        ciphertext.chars()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, c)| c)
            .collect::<String>()
    );
}

fn test_2() {
    let stdin = io::stdin();
    let mut data = stdin.lock().lines();

    let mut sales = vec![];
    for _ in 0..data.next().unwrap().unwrap().parse::<usize>().unwrap()  {
        sales.push(
            data.next()
                .unwrap()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        );
    }

    for sale in sales.chunks(7).fold([0; 7], |acc, x| array_plus(&acc, x)).iter() {
        println!("{}", sale);
    }
}

fn array_plus(a: &[usize], b: &[usize]) -> [usize; 7] {
    let mut array = [0; 7];
    for i in 0..7 { array[i] = a[i] + b[i]; }

    array
}

fn test_3() {
    println!("RENA");
}

fn test_4() {
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

    let mut sum = 0;
    for r in 1..h + 1 {
        let cs: Vec<_> = squares.iter()
            .filter(|v| r >= v[1] && r <= v[3])
            .map(|v| (v[0], v[2]))
            .collect();

        for c in 1..w + 1 {
            if cs.iter().any(|&(c1, c2)| c >= c1 && c <= c2) {
//                println!("{}, {}", r, c);
                sum += vals[r - 1][c - 1];
            }
        }
    }

    println!("{}", sum);
}

fn next_datum(l: &mut io::Lines<io::StdinLock>) -> Vec<usize> {
    l.next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}