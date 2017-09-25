use std::collections::HashMap;

fn main() {
    let map1 = "abcdefghijklmnopqrstuvwxyz";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz";

    let cipher = Cipher::new(map1, map2);

    let encode = cipher.encode("abc");

    println!("{}, {}", encode, cipher.decode(&encode));
}

struct Cipher {
    map_encode: HashMap<char, char>,
    map_decode: HashMap<char, char>
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        Cipher {
            map_encode: map1.chars().zip(map2.chars()).collect::<HashMap<char, char>>(),
            map_decode: map2.chars().zip(map1.chars()).collect::<HashMap<char, char>>()
        }
    }

    fn encode(&self, string: &str) -> String {
        if string.is_empty() { return String::from(""); }

        string.chars().map(|c| {
            if c.is_alphabetic() { self.map_encode.get(&c).unwrap() } else { c }
        }).collect()
    }

    fn decode(&self, string: &str) -> String {
        if string.is_empty() { return String::from(""); }

        string.chars().map(|c| {
            if c.is_alphabetic() { self.map_decode.get(&c).unwrap() } else { c }
        }).collect()
    }
}