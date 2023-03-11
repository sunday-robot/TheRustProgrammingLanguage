use crate::summary::Summary;

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize(100));
}
