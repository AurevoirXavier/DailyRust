fn main() {
    println!("{}", rank("Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin", vec![4, 2, 1, 4, 3, 1, 2], 4));

    println!("{}", rank("Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden", vec![1, 3, 5, 5, 3, 6], 2));

    println!("{}", rank("Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth", vec![3, 1, 4, 4, 3, 2], 4));

    println!("{}", rank("Lagon,Lily", vec![1, 5], 2));
}

fn rank(st: &str, we: Vec<i32>, n: usize) -> &str {
    if st.is_empty() { return "No participants"; };

    let st = st.split(',').collect::<Vec<&str>>();

    let names = st
        .iter()
        .map(|name: &&str| name.to_lowercase())
        .collect::<Vec<String>>();

    if names.len() < n { return "Not enough participants"; };

    let mut order: Vec<(&&str, i32)> = st
        .iter()
        .zip(names
            .iter()
            .map(|name: &String| -> i32 {
                let mut c = name.chars();

                let mut winning_number = 0;

                loop { if let Some(c) = c.next() { winning_number += (c as u8 - 96) as i32; } else { break; } }

                return (winning_number + name.len() as i32) * we[names.iter().position(|x: &String| x == name).unwrap()];
            })
        ).collect();

    order.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    order.reverse();

    let mut v = Vec::new();

    for (i, c) in order.iter().enumerate() { if c.1 == order[n - 1].1 { v.push(i); } }

    v.sort();

    order[v[0]..v[v.len() - 1] + 1].sort();

    println!("{:?}", order);

    return order[n - 1].0;
}
