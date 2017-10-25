fn main() {
    assert_eq!(
        recover_secret(
            vec![
                ['t', 'u', 'p'],
                ['w', 'h', 'i'],
                ['t', 's', 'u'],
                ['a', 't', 's'],
                ['h', 'a', 'p'],
                ['t', 'i', 's'],
                ['w', 'h', 's']
            ]
        ),
        "whatisup"
    );
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut todo: Vec<char> = triplets.iter().flat_map(|x| &x[..]).map(|&x| x).collect();

    todo.sort();
    todo.dedup();

    let mut done = vec![];

    while !todo.is_empty() {
        let i = (0..).find(|&i| {
            triplets.iter().all(|&c|
                (todo[i] != c[1] || done.contains(&c[0])) && (todo[i] != c[2] || done.contains(&c[1])))
        }).unwrap();

        done.push(todo.swap_remove(i));
    }

    done.into_iter().collect()
}

//fn recover_secret(triplets: Vec<[char; 3]>) -> String {
//    let mut result = String::new();
//    let mut triplets: Vec<Vec<char>> = triplets.into_iter().map(|triplet| triplet.to_vec()).collect();
//
//    while let Some(next) = get_next(&triplets) {
//        triplets = triplets
//            .iter()
//            .map(|column| column.iter().filter(|&c| *c != next).map(|&x| x).collect())
//            .collect()
//        ;
//
//        result.push(next);
//    }
//
//    result
//}
//
//fn get_next(triplets: &Vec<Vec<char>>) -> Option<char> {
//    let mut transform = vec![Vec::new(), Vec::new(), Vec::new()];
//
//    for triplet in triplets.iter() {
//        for (i, &c) in triplet.iter().enumerate() {
//            transform[i].push(c.clone());
//        }
//    }
//
//    transform[0]
//        .clone()
//        .into_iter()
//        .filter(|c| !transform[1].contains(c) && !transform[2].contains(c))
//        .next()
//}