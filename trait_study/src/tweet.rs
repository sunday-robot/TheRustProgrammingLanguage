//use trait_study::summary::Summary;

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl trait_study::summary::Summary for Tweet {
    fn summarize(&self, max_length:i32) -> String {
        format!(
            "USER NAME:[{}], CONTENT:[{}], [{}]",
            self.username,
            self.content,
            Tweet::summarize_util(max_length)
        )
    }
    fn summarize2(self) -> String {
        format!("{}", self.username)
    }
    fn summarize_util(x: i32) -> String {
        format!("{}", x)
    }
}
