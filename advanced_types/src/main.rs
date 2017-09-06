type Kilometers = i32;

type Thunk = Box<FnOnce() + Send + 'static>;

use std::io::Error;
use std::fmt;

type Result_<T> = Result<T, Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result_<usize>;

    fn flush(&mut self) -> Result_<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result_<()>;

    fn write_fmt(&mut self, fmt: fmt::Formatter) -> Result_<()>;
}

fn bar() -> ! {}

fn generic<T: ? Sized>(t: &T) {}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let _f: Thunk = Box::new(|| println!("hi"));
}
