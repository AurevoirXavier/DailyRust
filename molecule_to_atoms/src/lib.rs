/*
    "Mg(OH)2"
    => [("Mg",1),("O",2),("H",2)]

    "K4[ON(SO3)2]2"
    => [("K",4),("O",14),("N",2),("S",4)]
*/

type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut atoms: Vec<Atom> = Vec::new();
    let mut left_brackets: Vec<(char, usize)> = Vec::new();
    let mut indexes: Vec<usize> = Vec::new();

    let s: Vec<char> = s.chars().collect();

    for (i, c) in s.clone().into_iter().enumerate() {
        match c {
            'A' ... 'Z' => {
                atoms.push((c.to_string(), 1));
                indexes.push(i);
            }
            'a' ... 'z' => atoms.last_mut().unwrap().0.push(c),
            '2' ... '9' => {
                if s[i - 1].is_alphabetic() {
                    let str = get_num(i, &s);

                    atoms.last_mut().unwrap().1 = str.parse::<usize>().unwrap();
                }
            }
            '{' | '[' | '(' => left_brackets.push((c, i)),
            '}' | ']' | ')' => {
                let left_bracket = left_brackets.pop().unwrap();

                match c {
                    '}' => if left_bracket.0 != '{' { return Err(ParseError {}); }
                    ']' => if left_bracket.0 != '[' { return Err(ParseError {}); }
                    ')' => if left_bracket.0 != '(' { return Err(ParseError {}); }
                    _ => ()
                }
            }
            _ => return Err(ParseError {})
        }
    }

    println!("{:?}", atoms);

    Err(ParseError {})
}

fn get_num(i: usize, s: &Vec<char>) -> String {
    let mut j = i + 1;
    let mut str = s[i].to_string();

    while s[j].is_numeric() {
        str.push(s[j]);
        j += 1;
    }

    str
}
