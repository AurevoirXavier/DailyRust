#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:#?}", v);

    let v1 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v1[2];
    let third: Option<&i32> = v1.get(20);

    // let first = &v[0];
    // v.push(6);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
