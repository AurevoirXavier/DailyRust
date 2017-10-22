extern crate molecule_to_atoms;

use molecule_to_atoms::parse_molecule;

/*
    "Mg(OH)2"
    => [("Mg",1),("O",2),("H",2)]

    "K4[ON(SO3)2]2"
    => [("K",4),("O",14),("N",2),("S",4)]
*/

fn main() {
    let molecule = parse_molecule("C6H12O6");

    println!("molecule: {:?}", molecule);
}