fn main() {
    println!("{}", find_nb(24723578342962));
}

fn find_nb(n: u64) -> i32 {
    let sqrt = (1f64 + 8f64 * (n as f64).powf(0.5)).sqrt();

    if sqrt == sqrt.trunc() {
        return (-1 + sqrt as i32) / 2;
    } else {
        return -1;
    }
}