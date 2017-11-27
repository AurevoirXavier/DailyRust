fn main() {
    let mut v: Vec<u32> = vec![9, 1, 2, 5, 7, 4, 8, 6, 3, 5];

    shell_sort(&mut v);

    println!("{:?}", v);
}

/*
gqp = 5
9 1 2 5 7
4 8 6 3 5 -> 9 1 2 5 7 4 8 6 3 5

gap = 5
4 1 2 3 5
9 8 6 5 7 -> 4 1 2 3 5 9 8 6 5 7
---
gap = 2
4 1
2 3
5 9
8 6
5 7

gap = 2
2 1
4 3
5 6
5 7
8 9 -> 2 1 4 3 5 6 5 7 8 9
---
gap = 1
...
*/

fn shell_sort(ary: &mut Vec<u32>) {
    let len = ary.len();
    let mut gap = len / 2;

    while gap > 0 {
        for i in gap..len {
            //            let mut index = (i - gap) as i32;

            //            while index >= 0 {
            //                let j = index as usize;

            //                if  ary[j] > ary[j + gap] {
            //                    ary.swap(j, j + gap);

            //                    index -= gap as i32;
            //                } else {
            //                    break;
            //                }
            //            }

            for j in (0..i - gap + 1).rev() {
                if ary[j] > ary[j + gap] {
                    ary.swap(j, j + gap);
                } else {
                    break;
                }
            }
        }

        //        println!("{:?}", ary);

        gap /= 2;
    }
}

