macro_rules! what_is {
    (self) => {"the keyword `self`"};
    ($i:ident) => {concat!("the identifier `", stringify!($i), "`")};
}

macro_rules! call_with_ident {
    ($c:ident($i:ident)) => {$c!($i)};
}

fn main() {
    println!("{}", what_is!(self));
    println!("{}", call_with_ident!(what_is(self)));
}

//macro_rules! make_mutable {
//    ($i:ident) => {let mut $i = $i;};
//}
//
//struct Dummy(i32);
//
//impl Dummy {
//    fn double(self) -> Dummy {
//        make_mutable!(self);
//        self.0 *= 2;
//        self
//    }
//}
//
//macro_rules! make_self_mutable {
//    ($i:ident) => {let mut $i = self;};
//}
//
//struct Dummy(i32);
//
//impl Dummy {
//    fn double(self) -> Dummy {
//        make_self_mutable!(mut_self);
//        mut_self.0 *= 2;
//        mut_self
//    }
//}
//
//macro_rules! double_method {
//    ($body:expr) => {
//        fn double(mut self) -> Dummy {
//            $body
//        }
//    };
//}
//
//struct Dummy(i32);
//
//impl Dummy {
//    double_method! {{
//        self.0 *= 2;
//        self
//    }}
//}

macro_rules! double_method {
    ($self_:ident, $body:expr) => {
        fn double(mut $self_) -> Dummy {
            $body
        }
    };
}

struct Dummy(i32);

impl Dummy {
    double_method! {self, {
        self.0 *= 2;
        self
    }}
}

//macro_rules! double_method {
//    ($self_:ident, $body:expr) => {
//        fn double($self_) -> Dummy {
//            $body
//        }
//    };
//}
//
//struct Dummy(i32);
//
//impl Dummy {
//    double_method! {_, 0}
//}
