use std::fmt::{self, Formatter, Display};
use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

impl Matrix {
    fn transpose(&self) -> Matrix {
        Matrix(self.0, self.2, self.1, self.3)
    }
}

fn main() {
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    let transpose_matrix = matrix.transpose();
    println!("{}", transpose_matrix);

    let xs = [1, 2, 3, 4, 5];
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&xs[1 .. 4]);
}
