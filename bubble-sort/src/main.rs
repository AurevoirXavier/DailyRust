fn main() {
    let mut v: Vec<u32> = vec![5, 2, 4, 6, 1, 3];

    bubble_sort(&mut v);

    println!("{:?}", v);
}

fn bubble_sort(ary: &mut Vec<u32>) {
    //    let len = ary.len();

    //    for i in 0..len {
    //        for j in i..len {
    //            if ary[i] > ary[j] {
    //                ary.swap(i, j)
    //            }
    //        }
    //    }

    //    let mut loop_times = ary.len();
    //    let mut flag = true;

    //    while flag {
    //        flag = false;

    //        for i in 1..loop_times {
    //            if ary[i - 1] > ary[i] {
    //                ary.swap(i - 1, i);

    //                flag = true;
    //            }
    //        }

    //        loop_times -= 1;
    //    }

    let mut flag = ary.len();

    while flag > 0 {
        let loop_times = flag;

        flag = 0;

        for i in 1..loop_times {
            if ary[i - 1] > ary[i] {
                ary.swap(i - 1, i);

                flag = i;
            }
        }
    }
}
