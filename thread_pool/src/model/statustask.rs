#[derive(Debug, PartialEq, Eq)]
pub enum TaskStatus {
    Ready,
    Running,
    Waiting,
    Done,
    Error,
}
