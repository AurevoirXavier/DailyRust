fn main() {
    let code = ">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]";
    let v = boolfuck(code, b"Codewars\x00".to_vec());

    println!("{}", v.into_iter().map(|c| c as char).collect::<String>());
}

use std::collections::HashMap;

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let code = code.as_bytes();
    let mut input = input.into_iter().flat_map(|b| (0u8..8).map(move |i| (b >> i) & 1));
    let mut output = Vec::new();
    let mut tape = HashMap::new();
    let mut stack = Vec::new();
    let mut cp = 0;
    let mut tp = 0;

    while cp < code.len() {
        match code[cp] {
            b'+' => {
                let t = tape.entry(tp).or_insert(0);

                *t = if *t == 0 { 1 } else { 0 }
            }
            b',' => { input.next().map(|x| tape.insert(tp, x)); }
            b';' => output.push(*tape.get(&tp).unwrap_or(&0)),
            b'<' => tp -= 1,
            b'>' => tp += 1,
            b'[' => if *tape.get(&tp).unwrap_or(&0) == 0 { cp = matching_bracket(code, cp).unwrap(); } else { stack.push(cp); },
            b']' => cp = stack.pop().unwrap().wrapping_sub(1),
            _ => (),
        }
        cp = cp.wrapping_add(1);
    }

    output.chunks(8).map(|b| b.iter().rev().fold(0, |acc, x| (acc << 1) | x)).collect()
}

fn matching_bracket(code: &[u8], open: usize) -> Option<usize> {
    let mut stack = 0;

    for (i, &c) in code[open..].iter().enumerate() {
        match c {
            b'[' => stack += 1,
            b']' => {
                stack -= 1;

                if stack == 0 { return Some(open + i); }
            }
            _ => (),
        }
    }

    None
}

//fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
//    let code = code.as_bytes();
//
//    let mut tap = [false; 5000];
//    let mut code_ptr = 0;
//    let mut tap_ptr = 2500;
//    let mut output_stream = Vec::new();
//    let input_stream: Vec<bool> = {
//        let mut string = String::new();
//
//        for char in input { string += &format!("{:08b}", char); }
//
//        string.chars().collect::<Vec<char>>()
//            .chunks_mut(8)
//            .map(|chunk| chunk.iter().rev().collect::<String>())
//            .collect::<String>()
//            .chars().map(|c| if c == '1' { true } else { false })
//            .collect()
//    };
//    let mut input = input_stream.into_iter();
//
//
//    while code_ptr < code.len() {
//        match code[code_ptr] {
//            b'>' => tap_ptr += 1,
//            b'<' => tap_ptr -= 1,
//            b'+' => if tap[tap_ptr] { tap[tap_ptr] = false; } else { tap[tap_ptr] = true; },
//            b';' => output_stream.push(tap[tap_ptr]),
//            b',' => if let Some(input) = input.next() { tap[tap_ptr] = input; },
//            b'[' if !tap[tap_ptr] => code_ptr += jump(code[code_ptr..].iter()),
//            b']' if tap[tap_ptr] => code_ptr -= jump(code[0..code_ptr + 1].iter().rev()),
//            _ => {}
//        }
//
//        code_ptr += 1;
//    }
//
//    let mut chunks = output_stream.chunks(8);
//    let mut output = Vec::new();
//
//    while let Some(chunk) = chunks.next() {
//        let mut chunk = chunk.to_vec();
//        let mut char = 0b00000000u8;
//
//        while chunk.len() != 8 { chunk.push(false); }
//
//        for (carry_bit, flag) in chunk.iter().enumerate() {
//            if *flag { char += 0b00000001u8 << carry_bit }
//        }
//
//        output.push(char);
//    }
//
//    output
//}
//
//fn jump<'a, I: 'a>(code: I) -> usize where I: Iterator<Item=&'a u8> {
//    let mut n = 0;
//
//    for (i, &c) in code.enumerate() {
//        if c == b'[' { n += 1; }
//        if c == b']' { n -= 1; }
//        if n == 0 { return i; }
//    }
//
//    unreachable!();
//}