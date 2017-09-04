pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait Summarizable2 {
    fn author_summary(&self) -> String;

    fn summary2(&self) -> String {
        format!("(Read more form {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summarizable2 for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
