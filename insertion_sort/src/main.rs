fn main() {
    let mut v: Vec<u32> = vec![5, 2, 4, 6, 1, 3];

    insertion_sort(&mut v);

    println!("{:?}", v);
}

fn insertion_sort(ary: &mut Vec<u32>) {
    for i in 1..ary.len() {
        //        let key = ary[i];

        //        let mut j = i;

        //        while j > 0 && ary[j - 1] > key {
        //            ary[j] = ary[j - 1];

        //            ary[j - 1] = key;

        //            j -= 1;
        //        }

        for j in (0..i).rev() {
            if ary[j] > ary[j + 1] {
                ary.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
}