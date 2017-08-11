// 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、
// 中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；
// 这里哈希函数会很有帮助）。

// 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到
// 单词的结尾并增加“ay”，所以“first”会变成“irst-fay”。
// 元音字母开头的单词则在结尾增加 “hay”（“apple”会变成“apple-hay”）。
// 牢记 UTF-8 编码！

// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中
// 增加员工的名字。例如，“Add Sally to Engineering”或“Add Amir to Sales”。
// 接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工
// 按照字母顺排序的列表。

fn main() {
    let mut v: Vec<i32> = vec![2, 3, 5, 1, 2, 3, 8, 6, 3, 1, 4, 6, 7];

    let mut sum = 0;

    let len = v.len();

    for element in &v {
        sum += *element;
    }

    println!("average: {}", (sum as f64) / (len as f64),);    

    v.sort();

    use std::collections::HashMap;

    let mut h: HashMap<i32, i32> = HashMap::new();

    println!("mid: {}", &v[len / 2]);

    for element in v {
        let count = h.entry(element).or_insert(0);

        *count += 1;
    }

    let mut most: Vec<i32> = Vec::new();

    let mut max: i32 = 0;

    for v in h.values() {
        if v > &max {
            max = *v;
        }
    }

    for (k, v) in h {
        if v == max {
            most.push(k);
        }
    }

    println!("most: {:?}", most);
}
