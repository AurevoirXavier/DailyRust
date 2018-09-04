#[derive(Debug)]
pub struct Kyu {
    address: String,
    name: String,
    project: String,
    path: String,
    description: String,
}

impl Kyu {
    pub fn new(address: String) -> Kyu {
        Kyu {
            address,
            name: String::new(),
            project: String::new(),
            path: String::new(),
            description: String::new(),
        }
    }

    pub fn war_time(&mut self) {
        self.parse();
        self.write();
    }

    fn parse(&mut self) {
        use regex::Regex;
        use reqwest::get;
        use select::document::Document;
        use select::predicate::{Predicate, Class, Name};
        use serde_json;

        let text = get(&self.address)
            .expect("an error occur while downloading")
            .text()
            .unwrap();
        let document = Document::from(text.as_str());

        {
            let rank = document.find(Class(
                "inner-small-hex"
            ).descendant(Name(
                "span"
            ))).next()
                .unwrap()
                .text();
            let re = Regex::new(r"kata/(.+)/train").unwrap();
            self.project = re.captures(&self.address)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            self.path = format!("{}/{}", rank, &self.project);
        }

        let mut data = document.find(Name("script"));
        for _ in 0..8 { data.next(); }
        let text = data.next()
            .unwrap()
            .text();

        let re = Regex::new(r"App\.data = (.+)\nApp\.routes").unwrap();
        let caps = re.captures(&text).unwrap();
        let v: serde_json::Value = serde_json::from_str(
            caps.get(1)
                .unwrap()
                .as_str()
        ).unwrap();

        let name = v["challengeName"].to_string();
        self.name = Kyu::remove_quotes(name);

        let description = v["description"].to_string();
        self.description = Kyu::remove_quotes(description);
    }

    fn remove_quotes(text: String) -> String { text[1..text.len() - 1].to_string() }

    fn write(&self) {
        {
            use std::fs::create_dir_all;
            create_dir_all(&self.path).expect("failed to create dir");
        }

        {
            use std::env::set_current_dir;
            use std::path::Path;
            set_current_dir(Path::new(&self.path)).expect("failed to change work dir");
        }

        println!("{:?}", self);

        {
            use std::process::Command;
            Command::new("cargo")
                .args(&["init", "--name", &self.project])
                .output()
                .expect("failed to init project");
        }

        {
            use std::fs::File;
            use std::io::prelude::*;
            let mut f = File::create("README.md").expect("failed to create README.md");
            f.write_all(self.description.as_bytes()).expect("an error occur while writing");
            f.sync_all().expect("an error occur while sync(ing) data");
        }
    }
}