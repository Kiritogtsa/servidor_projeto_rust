use super::{status::Statusbase, task::Task};

pub trait Workertrait<T> {
    fn get_len(&self) -> usize;
    fn run(&mut self, task: usize);
    fn set_status(&mut self, status: Statusbase);
    fn status(&self) -> Statusbase;
    fn geturl(&self) -> T;
}
