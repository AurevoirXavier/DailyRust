#![feature(test)]
extern crate test;

use test::Bencher;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(2279184, dfs_5(0, 0, 0, 0));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(0);
            dfs_5(n, 0, 0, 0)
        })
    }
}

const N: u8 = 15;

// #################################################################################################
fn solution_1() -> u32 {
    let mut count = 0u32;
    let mut rows = [0; N as usize];

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
    let mut columns = [false; N as usize];
    let mut left_diagonal = [false; 2 * (N as usize) - 1];
    let mut right_diagonal = [false; 2 * (N as usize) - 1];

    dfs_2(0, &mut count, &mut columns, &mut left_diagonal, &mut right_diagonal);

    count
}

fn dfs_2(row: u8, count: &mut u32, columns: &mut [bool], left_diagonal: &mut [bool], right_diagonal: &mut [bool]) {
    for column in 0..N {
        let (j, k) = ((row + column) as usize, (N - 1 + column - row) as usize);

        if columns[column as usize] || left_diagonal[j] || right_diagonal[k] { continue; }

        if row == N - 1 { *count += 1; } else {
            columns[column as usize] = true;
            left_diagonal[j] = true;
            right_diagonal[k] = true;

            dfs_2(row + 1, count, columns, left_diagonal, right_diagonal);

            columns[column as usize] = false;
            left_diagonal[j] = false;
            right_diagonal[k] = false;
        }
    }
}
// #################################################################################################

// #################################################################################################
fn solution_3() -> u32 {
    let mut count = 0u32;
    let mut columns = 0u32;
    let mut left_diagonal = 0u32;
    let mut right_diagonal = 0u32;

    dfs_3(0, &mut count, &mut columns, &mut left_diagonal, &mut right_diagonal);

    count
}

fn dfs_3(row: u8, count: &mut u32, columns: &mut u32, left_diagonal: &mut u32, right_diagonal: &mut u32) {
    for column in 0..N {
        let (j, k) = (row + column, N - 1 + column - row);

        if (((*columns >> column) | (*left_diagonal >> j) | (*right_diagonal >> k)) & 1) != 0 { continue; }

        if row == N - 1 { *count += 1; } else {
            *columns ^= 1 << column;
            *left_diagonal ^= 1 << j;
            *right_diagonal ^= 1 << k;

            dfs_3(row + 1, count, columns, left_diagonal, right_diagonal);

            *columns ^= 1 << column;
            *left_diagonal ^= 1 << j;
            *right_diagonal ^= 1 << k;
        }
    }
}
// #################################################################################################

// #################################################################################################
fn solution_4() -> u32 {
    let mut count = 0u32;
    let mut columns = 0u32;
    let mut left_diagonal = 0u32;
    let mut right_diagonal = 0u32;

    dfs_4(0, &mut count, &mut columns, &mut left_diagonal, &mut right_diagonal);

    count
}

fn dfs_4(row: u8, count: &mut u32, columns: &mut u32, left_diagonal: &mut u32, right_diagonal: &mut u32) {
    let mut available = ((1 << N) - 1) & !(*columns | (*left_diagonal >> row) | (*right_diagonal >> (N - 1 - row)));

    while available != 0 {
        let position = available & (!available + 1);

        available ^= position;

        if row == N - 1 { *count += 1; } else {
            *columns ^= position;
            *left_diagonal ^= position << row;
            *right_diagonal ^= position << (N - 1 - row);

            dfs_4(row + 1, count, columns, left_diagonal, right_diagonal);

            *columns ^= position;
            *left_diagonal ^= position << row;
            *right_diagonal ^= position << (N - 1 - row);
        }
    }
}
// #################################################################################################

// #################################################################################################
fn solution_5() -> u32 {
    dfs_5(0, 0, 0, 0)
}

fn dfs_5(row: u8, columns: u32, left_diagonal: u32, right_diagonal: u32) -> u32 {
    let mut count = 0;
    let mut available = ((1 << N) - 1) & !(columns | left_diagonal | right_diagonal);

    while available != 0 {
        let position = available & (!available + 1);

        available ^= position;

        if row == N - 1 { count += 1; } else {
            count += dfs_5(row + 1, columns ^ position, (left_diagonal ^ position) >> 1, (right_diagonal ^ position) << 1);
        }
    }

    count
}
// #################################################################################################
