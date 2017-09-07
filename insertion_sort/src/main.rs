fn main() {
    let mut v: Vec<u32> = vec![8, 4, 6, 2, 10];

    insertion_sort(&mut v);

    println!("{:?}", v);
}

fn insertion_sort(ary: &mut Vec<u32>) {
    let len = ary.len();

    for i in 0..len {
        for j in i..len {
            if ary[j] > ary[i] {
                let tmp = ary[j];

                ary[j] = ary[i];
                ary[i] = tmp;
            }
        }
    }
}