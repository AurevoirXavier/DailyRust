fn main() {
    println!("{}", longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2));
}

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut result = String::new();

    if k > 0 && strarr.len() >= k {
        for index in 0..strarr.len() - k + 1 {
            let s: String = strarr[index..index + k].join("");

            if s.len() > result.len() { result = s; }
        }
    }

    result
}

//fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
//    let len = strarr.len();
//
//    if len == 0 || k > len || k <= 0 { return String::from("") }
//
//    let mut candidate: Vec<(usize, String)> = Vec::new();
//
//    for i in 0..len - k + 1 {
//        let mut result = String::new();
//
//        for j in 0..k { result += strarr[i + j]; }
//
//        candidate.push((result.len(), result));
//    }
//
//    let max = candidate.iter().max().unwrap().0;
//
//    candidate.retain(|x| x.0 == max);
//
//    let candidate = candidate.iter().map(|x| &x.1).collect::<Vec<&String>>();
//
//    candidate[0].to_string()
//}