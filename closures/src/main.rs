fn apply<F>(f: F) where F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

fn apply_type_anonymity<F>(f: F) where F: Fn() {
    f();
}

fn call_me<F: Fn()>(f: F) {
    f()
}

fn function() {
    println!("I'm a function!");
}

fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

fn create_fn_mut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();

    Box::new(move || println!("This is a: {}", text))
}

//fn create_fn_once() -> Box<FnOnce()> {
//    let text = "FnOnce".to_owned();
//
//    Box::new(move || println!("This is a: {}", text))
//}


fn main() {
    let add_one = |x| x + 1;

    let five = add_one(4);

    assert_eq!(5, five);

    let calculate = |a, b| {
        let mut result = a * 2;

        result += b;

        result
    };

    assert_eq!(7, calculate(2, 3));
    assert_eq!(13, calculate(4, 5));

    let add_one = |x: i32| -> i32 {
        x + 1
    };

    assert_eq!(2, add_one(1));

    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));

    fn call_with_one<F>(some_closure: F) -> i32 where F: Fn(i32) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);

    assert_eq!(3, answer);

    let color = "green";
    let print = || println!("`color`: {}", color);

    print();
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;

        println!("`count`: {}", count);
    };

    inc();
    inc();

    use std::mem;

//    let not_non_copy = 3;
    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);

        mem::drop(movable);
//        mem::drop(not_non_copy);
    };

    consume();
//    consume();

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
//    println!("There're {} elements in vec", haystack.len());

    let bigger = |a: u32, b: u32| if a > b { a } else { b };

    println!("{}", bigger(1, 2));

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");

        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    let x = 7;
    let print = || println!("{}", x);

    apply_type_anonymity(print);

    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    let fn_plain = create_fn();
    let mut fn_mut = create_fn_mut();
//    let fn_once = create_fn_once();

    fn_plain();
    fn_mut();
//    fn_once();

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}
