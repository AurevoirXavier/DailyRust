extern crate molecule_to_atoms;

use molecule_to_atoms::parse_molecule;

fn main() {
    let molecule = parse_molecule("K4[ON(SO3)2]2");

    println!("molecule: {:?}", molecule);
}