fn main() {
    dont_give_me_five(1, 500);
}

fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..end+1)
        .filter(|x| !x.to_string().contains('5'))
        .count() as isize
}

//fn dont_give_me_five(start: isize, end: isize) -> isize {
//    let mut count = 0;
//
//    for num in start..end + 1 {
//        let mut flag = true;
//
//        let string = num.to_string();
//
//        for char in string.chars() {
//            if char == '5' {
//                flag = false;
//
//                break;
//            }
//        }
//
//        if flag {
//            count += 1;
//
//            print!("{} ", num);
//        }
//    }
//
//    println!();
//
//    count
//}