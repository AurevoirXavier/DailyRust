//#![feature(test)]
//extern crate test;
//
//use test::Bencher;
//
//#[bench]
//fn bench_solution(b: &mut Bencher) {
//    b.iter(|| test::black_box(solution_1()));
//}

const N: u8 = 12;

fn main() {
//    println!("{}", solution_1());
    println!("{}", solution_2());
}

// #################################################################################################
fn solution_1() -> u32 {
    let mut count = 0u32;
    let mut rows = [0; N as usize + 1];

    dfs_1(0, &mut count, &mut rows);

    count
}

fn dfs_1(row: u8, count: &mut u32, rows: &mut [u8]) {
    for column in 0..N {
        let mut flag = true;

        for i in 0..row {
            if column == rows[i as usize] || column as i8 == rows[i as usize] as i8 - row as i8 + i as i8 || column as i8 - row as i8 + i as i8 == rows[i as usize] as i8 {
                flag = false;

                break;
            }
        }

        if !flag { continue; }

        rows[row as usize] = column;

        if row == N - 1 { *count += 1; } else { dfs_1(row + 1, count, rows); }
    }
}
// #################################################################################################

// #################################################################################################
fn solution_2() -> u32 {
    let mut count = 0u32;
    let mut rows = [0; N as usize + 1];

    dfs_2(0, &mut count, &mut rows);

    count
}

fn dfs_2(row: u8, count: &mut u32, rows: &mut [u8]) {
    for column in 0..N {
        let mut flag = true;

        for i in 0..row {
            if column == rows[i as usize] || column as i8 == rows[i as usize] as i8 - row as i8 + i as i8 || column as i8 - row as i8 + i as i8 == rows[i as usize] as i8 {
                flag = false;

                break;
            }
        }

        if !flag { continue; }

        rows[row as usize] = column;

        if row == N - 1 { *count += 1; } else { dfs_1(row + 1, count, rows); }
    }
}
// #################################################################################################

