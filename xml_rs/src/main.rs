extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::cmp::Ordering;

use xml::reader::{EventReader, XmlEvent};

//#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Book {
    title: String,
    price: u32,
}

impl Ord for Book {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.price).cmp(&(other.price))
    }
}

impl PartialOrd for Book {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        (self.price, &self.title) == (other.price, &other.title)
    }
}

impl Eq for Book {}

fn parse_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut books = Vec::new();
    let mut current_element = String::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "book" {
                    let book = Book {
                        title: String::new(),
                        price: 0,
                    };
                    books.push(book);
                }
                current_element = name.local_name;
            }
            Ok(XmlEvent::Characters(s)) => {
                match current_element.as_ref() {
                    "title" => {
                        let book = books.last_mut().unwrap();
                        book.title = s;
                    }
                    "price" => {
                        let book = books.last_mut().unwrap();
                        book.price = s.parse::<u32>().unwrap();
                    }
                    _ => { println!("no match"); }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    books
}

fn main() {
    let mut books = parse_books("book.xml");

    books.sort();

    println!("All books:");

    for book in &books {
        println!("Name: {}, Price: {}", book.title, book.price);
    }

    let gt_thirty = books.iter().filter(|book| book.price > 30);

    println!("\nPrice great than 30:");

    for book in gt_thirty {
        println!("Name: {}, Price: {}", book.title, book.price);
    }
}