fn main() {
    println!("{:?}", divisors(12));
    println!("{:?}", divisors(13));
}

fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut divisors = Vec::new();

    let square_root = (integer as f64).sqrt() as u32 + 1;

    for num in 2..square_root {
        if integer % num == 0 && num * num != integer {
            divisors.push(num);
            divisors.push(integer / num);
        }
        if integer % num == 0 && num * num == integer {
            divisors.push(num);
        }
    }

    divisors.sort();

    if divisors.is_empty() { return Err(integer.to_string() + " is prime"); } else { return Ok(divisors); }
}

//fn divisors(integer: u32) -> Result<Vec<u32>, String> {
//    let result: Vec<u32> = (2..integer).filter(|x| integer % x == 0).collect();
//    if result.len() > 0 { Ok(result) } else { Err(format!("{} is prime", integer)) }
//}
