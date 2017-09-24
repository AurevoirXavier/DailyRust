fn main() {
    println!("{}", prime_factors(86240));
}

fn prime_factors(n: i64) -> String {
    let mut v = Vec::new();
    let mut num = n;

    for i in 2..n {
        while num % i == 0 {
            num /= i;

            if !v.is_empty() { if v.last().unwrap() != &i { v.push(0); } }

            v.push(i);
        }

        if num == 1 { break; }
    }

    if v.is_empty() { return format!("({})", n); }

    let v = v.split(|x| x == &0).filter(|v| !v.is_empty()).collect::<Vec<&[i64]>>();

    let mut result = String::new();

    for v in v {
        if v.len() != 1 {
            result += &format!("({}**{})", v.first().unwrap(), v.len());
        } else {
            result += &format!("({})", v.first().unwrap());
        }
    }

    result
}