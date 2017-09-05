static HELLO_WORLD: &str = "Hello, world";

static mut COUNTER: u32 = 0;

unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345;
    let _r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    //    use std::slice;

    //    let slice = unsafe {
    //        slice::from_raw_parts_mut(r, 10000)
    //    };

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

//use std::slice;

//fn split_at_mut_(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//    let len = slice.len();
//
//    let ptr = slice.as_mut_ptr();
//
//    assert!(mid <= len);
//
//    unsafe {
//        (slice::from_raw_parts_mut(ptr, mid), slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
//    }
//}

//extern "C" {
//    fn some_function();
//}

//fn main() {
//    unsafe { some_function() };
//}

//#[no_mangle]
//pub extern "C" fn call_from_c() {
//    println!("Just called a Rust function from C!");
//}