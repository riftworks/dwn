#[get("/echo")]
pub fn echo() -> &'static str {
    "hello"
}
