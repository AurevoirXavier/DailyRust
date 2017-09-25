fn main() {
    println!("{}", get_middle("test"));
}

fn get_middle(s: &str) -> &str {
    let len = s.len();

    if len % 2 == 0 { return &s[len / 2 - 1..len / 2 + 1]; } else { return &s[len / 2 - 1..len / 2]; }
}
