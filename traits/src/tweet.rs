use crate::summary::Summary;

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Tweet {
    pub fn new(username: &str, content: &str) -> Tweet {
        Tweet {
            username: String::from(username),
            content: String::from(content),
            reply: false,
            retweet: false,
        }
    }
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
