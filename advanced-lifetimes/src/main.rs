trait Foo {}

struct _Context<'s>(&'s str);

struct _Parser<'c, 's: 'c> {
    context: &'c _Context<'s>
}

struct _Ref<'a, T: 'a>(&'a T);

struct _StaticRef<T: 'static>(&'static T);

struct Bar<'a> {
    _x: &'a i32
}

impl<'c, 's> _Parser<'c, 's> {
    fn _parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

impl<'a> Foo for Bar<'a> {}

fn _parse_context(context: _Context) -> Result<(), &str> {
    _Parser { context: &context }._parse()
}

fn main() {
    let num = 5;

    let _obj = Box::new(Bar { _x: &num }) as Box<Foo>;

    //    let _bound = Box::new(Bar { _x: &num}) as Box<Foo + 'static>;
}
