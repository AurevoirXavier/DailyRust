fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }

    println!("LIFTOFF!!!");    
}
