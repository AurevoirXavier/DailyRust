fn main() {
    let day = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];
    let lrc = [
        "A Partridge in a Pear Tree.",
        "2 Turtle Doves",
        "3 French Hens",
        "4 Calling Birds",
        "5 Golden Rings",
        "6 Geese a Laying",
        "7 Swans a Swimming",
        "8 Maids a Milking",
        "9 Ladies Dancing",
        "10 Lords a Leaping",
        "11 Pipers Piping",
        "12 Drummers Drumming",
    ];

    for i in 0..12 {
        print!(
            "On the {} day of Chirstmas my true love sent to me: ",
            day[i]
        );

        match i {
            0 => println!("{}\n", lrc[i]),
            _ => {
                // let mut temp = "".to_string();
                let mut temp = String::from("");

                for j in (0..i + 1).rev() {
                    if temp != "" {
                        if j == 0 {
                            // temp = temp + &", and a Partridge in a Pear Tree.".to_string();
                            temp.push_str(", and a Partridge in a Pear Tree.");                            
                        } else {
                            // temp = temp + &", ".to_string() + &lrc[j].to_string();
                            temp.push_str(", ");
                            temp.push_str(lrc[j]);                            
                        }
                    } else if temp == "" {
                        // temp = lrc[j].to_string();
                        temp = lrc[j].to_string();                        
                    }
                }

                println!("{}\n", temp);
            }
        }
    }
}
