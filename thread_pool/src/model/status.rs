pub enum Statusbase {
    Ready,
    Running,
    Waiting,
    Done,
    Error(String),
}
