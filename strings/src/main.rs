fn main() {
    let data = "initial contents";

    {
        let s = data.to_string();

        let s = "initial contents".to_string();

        let s = String::from("initial contents");
    }

    {
        let mut s = String::from("lo");
        s.push('l');

        println!("{}", s);
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;

        println!("{}", s3);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);

        println!("{}", s);
    }

    {
        let hello = "Здравствуйте";

        let s = &hello[0..4];

        println!("{}", s);
    }

    {
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }

        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}
