use crate::summary_trait::SummaryTrait;

pub fn notify(item: &impl SummaryTrait) {
    println!("Breaking news! {}", item.summarize(100));
}
