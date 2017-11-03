fn main() {
    //    let test = String::from_utf8(
    //        brain_luck(
    //            ",+[-.,+]"
    //            , ez_vec("Codewars", 255)
    //        )
    //    ).unwrap();

    let test = brain_luck(
        ",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.,"
        , vec![0, 4]
    );

    println!("{:?}", test);
}

fn ez_vec(str: &str, terminating_byte: u8) -> Vec<u8> {
    let mut v8 = str.chars().collect::<String>().into_bytes();

    v8.push(terminating_byte);

    v8
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let code = code.as_bytes();

    let mut input = input.into_iter();
    let mut output = vec![];
    let mut data = [0u8; 5000];
    let mut cp = 0;
    let mut dp = 0;

    while cp < code.len() {
        match code[cp] {
            b'>' => dp += 1,
            b'<' => dp -= 1,
            b'+' => data[dp] = data[dp].wrapping_add(1),
            b'-' => data[dp] = data[dp].wrapping_sub(1),
            b'.' => output.push(data[dp]),
            b',' => if let Some(input) = input.next() { data[dp] = input },
            b'[' if data[dp] == 0 => cp += jump(code[cp..].iter()),
            b']' if data[dp] != 0 => cp -= jump(code[0..cp + 1].iter().rev()),
            _ => {}
        }

        cp += 1;
    }

    output
}

fn jump<'a, I: 'a>(code: I) -> usize
    where I: Iterator<Item=&'a u8>
{
    let mut n = 0;

    for (i, &c) in code.enumerate() {
        if c == b'[' { n += 1; }
        if c == b']' { n -= 1; }
        if n == 0 { return i; }
    }

    unreachable!();
}

//fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
//    let mut bf = Brainfuck::new(code, input);
//
//    bf.compile();
//    bf.output
//}
//
//struct Brainfuck {
//    code: Vec<char>,
//    input: Vec<u8>,
//    output: Vec<u8>,
//    pointer: usize,
//    mem: Vec<u8>,
//}
//
//impl Brainfuck {
//    fn new(code: &str, input: Vec<u8>) -> Brainfuck {
//        Brainfuck {
//            code: code.chars().filter(|&c|
//                match c {
//                    '+' | '-' | '>' | '<' | ',' | '.' | '[' | ']' => true,
//                    _ => false
//                }
//            ).collect::<Vec<char>>(),
//            input,
//            output: Vec::new(),
//            pointer: 0,
//            mem: vec![0],
//        }
//    }
//
//    fn dot(&mut self) { self.output.push(self.mem[self.pointer]); }
//
//    fn comma(&mut self) { if let Some(value) = self.input.pop() { self.mem[self.pointer] = value; } }
//
//    fn lt(&mut self) { if self.pointer != 0 { self.pointer -= 1; } }
//
//    fn gt(&mut self) {
//        self.pointer += 1;
//
//        if self.mem.len() < self.pointer + 1 { self.mem.push(0); }
//    }
//
//    fn plus(&mut self) {
//        let value = self.mem[self.pointer];
//
//        if value != 255 { self.mem[self.pointer] += 1; } else { self.mem[self.pointer] = 0; }
//    }
//
//    fn minus(&mut self) {
//        let value = self.mem[self.pointer];
//
//        if value != 0 { self.mem[self.pointer] -= 1; } else { self.mem[self.pointer] = 255; }
//    }
//
//
//    fn compile(&mut self) {
//        self.input.reverse();
//
//        let mut index: usize = 0;
//        let mut inner_pass: usize = 0;
//        let mut pass = false;
//        let mut left_brackets_indexes = Vec::new();
//
//        let op = self.code.clone();
//
//        while op.len() > index {
//            let char = op[index];
//
//            match char {
//                char if pass && char != ']' && char != '[' => {
//                    index += 1;
//                    continue;
//                }
//                '+' => self.plus(),
//                '-' => self.minus(),
//                '>' => self.gt(),
//                '<' => self.lt(),
//                '.' => self.dot(),
//                ',' => self.comma(),
//                '[' =>
//                    if !pass {
//                        if self.mem[self.pointer] != 0 { left_brackets_indexes.push(index); } else { pass = true; }
//                    } else { inner_pass += 1; },
//                ']' =>
//                    if pass {
//                        if inner_pass > 0 { inner_pass -= 1; } else { pass = false; }
//                    } else {
//                        index = left_brackets_indexes.pop().unwrap();
//                        index -= 1;
//                    },
//                _ => unreachable!()
//            }
//
//            index += 1;
//        }
//    }
//}