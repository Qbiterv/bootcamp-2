#[ic_cdk::query]
fn greet(name: String, birth_day: i8) -> String {
    format!("Hello, {} {}!", name, birth_day)
}
