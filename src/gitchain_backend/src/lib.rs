#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn simple_greet() -> String {
    format!("Hello from Ahmed!")
}

