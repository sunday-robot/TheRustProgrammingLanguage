use crate::summary_trait::SummaryTrait;

// Javaなら、"public class NewArticle implemtns Summarizable {..."などとするが、
// RustではSummaryトレイトについてはここでは記述せず、後で別に記述する。
#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Javaなら、"public class NewArticle implemtns Summarizable {..."などとするが、
// RustではNewArticleの定義とは別に記述する。
// Javaの様にNewArticleの定義として、Summaryを実装する旨を記述するほうが良いように思うが、どういう意図があるのだろう?
impl SummaryTrait for NewsArticle {
    // Javaなどの場合は普通のメソッドと、クラスメソッドは、クラスメソッド側に"static"を付けて区別する。
    // Rustでは逆に、普通のメソッド側の第1引数に"&self"を記述することで区別する。
    // 暗黙のthisは書いたりしないが、Rustではselfを明記する。このため、"&self"は、"&this"などとしてはいけない…
    // おそらくオブジェクト指向を後から取り入れたPythonがこんな感じだったとおもうが、Pythonの方が自然だったと思う。
    fn summarize(&self, max_length:i32) -> String {
        format!("{}, by {} ({}) {} {}", self.headline, self.author, self.location, self.content, max_length)
    }
    fn summarize_util(x: i32) -> String {
        format!("{}", x + 1)
    }
}
