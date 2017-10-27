fn main() {
    println!("{}", mix("codewars", "codewars"));
}

fn mix(s1: &str, s2: &str) -> String {
    let mut s1: Vec<char> = s1.chars().filter(|c| c.is_alphabetic() && c.is_lowercase()).collect();
    let mut s2: Vec<char> = s2.chars().filter(|c| c.is_alphabetic() && c.is_lowercase()).collect();

    s1.sort();
    s2.sort();

    let s1 = split_diff(&s1, &get_first(&s1));
    let s2 = split_diff(&s2, &get_first(&s2));

    let common: Vec<(String, usize)> = s1.iter().map(|s1| {
        let mut len = s1.len();
        let mut s = format!("/1:{}", s1);

        if s2.iter().any(|s2| {
            let flag = s2.chars().next() == s1.chars().next();

            if flag == false { return false; }

            let len1 = s1.len();
            let len2 = s2.len();

            if len1 < len2 {
                len = len2;
                s = format!("/2:{}", s2);
            } else if len == len2 { s = format!("/=:{}", s1); }

            flag
        }) { (s, len) } else { (String::new(), 0) }
    }).filter(|&(_, len)| len > 1).collect();

    let different = [get_difference(&s1, &common).into_iter().map(|(s, len)| (format!("/1:{}", s), len)).collect::<Vec<(String, usize)>>(), get_difference(&s2, &common).into_iter().map(|(s, len)| (format!("/2:{}", s), len)).collect::<Vec<(String, usize)>>()].concat();

    let mut result = [common, different].concat();

    if result.is_empty() { return String::new(); }

    result.sort_by(|&(ref s1, len1), &(ref s2, len2)| {
        if len1 != len2 { len2.cmp(&len1) } else { s1.cmp(&s2) }
    });

    result.into_iter().map(|(string, _)| string).collect::<Vec<String>>().join("")[1..].to_string()
}

fn get_first(s: &Vec<char>) -> Vec<char> {
    let mut first = vec![s[0]];

    for i in 1..s.len() { if s[i - 1] != s[i] { first.push(s[i]); } }

    first
}

fn split_diff(s: &Vec<char>, first: &Vec<char>) -> Vec<String> {
    let mut split = Vec::new();

    for first in first.iter() { split.push(s.iter().filter(|c| c == &first).map(|c| *c).collect::<String>()) }

    split
}

fn get_difference(s: &Vec<String>, same: &Vec<(String, usize)>) -> Vec<(String, usize)> {
    s.iter().map(|s| { if same.iter().any(|&(ref same, _)| same.contains(s)) { (String::new(), 0) } else { (s.to_string(), s.len()) } }).filter(|&(_, len)| len > 1).collect()
}
