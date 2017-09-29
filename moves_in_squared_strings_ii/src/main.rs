fn main() {
    println!("{}", oper(rot, "abcd\nefgh\nijkl\nmnop"));
    println!("{}", oper(selfie_and_rot, "abcd\nefgh\nijkl\nmnop"));
}

fn rot(s: &str) -> String {
    s.chars().rev().collect()
}

fn selfie_and_rot(s: &str) -> String {
    s.lines()
        .map(|line| {
            line.to_string() + &dots(line.len()) + "\n"
        })
        .collect::<String>()
        +
        &rot(s).lines()
            .map(|line| {
                dots(line.len()) + line
            })
            .collect::<Vec<String>>()
            .join("\n")
}

//fn dots(mut len: usize) -> String {
//    let mut dots = String::from("");
//
//    while len != 0 {
//        dots += ".";
//
//        len -= 1;
//    }
//
//    dots
//}

fn dots(len: usize) -> String {
    std::iter::repeat(".").take(len).collect()
}

fn oper(f: fn(&str) -> String, s: &str) -> String {
    f(s)
}
