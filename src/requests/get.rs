#[get("/hello/<name>")]
pub fn index(name: &str) -> String {
    format!("Hello, {name}!")
}