pub enum Body<T> {
    String(String),
    Json(T),
}
