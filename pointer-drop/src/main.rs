struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer!");
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data")
    };

    println!("CustomSmartPointer created.");
    println!("Wait for it...");
}
