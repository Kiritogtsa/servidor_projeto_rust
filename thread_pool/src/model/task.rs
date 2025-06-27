use super::status::Statusbase;

pub trait Task {
    fn run(&mut self) -> Statusbase;
    fn status(&self) -> Statusbase;

    fn stop(&mut self);
}
