use super::statustask::TaskStatus;
pub trait Task {
    fn run(&self) -> TaskStatus;
    fn status(&self) -> TaskStatus;
    
    fn stop(&mut self);
}
