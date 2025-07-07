use super::{status::Statusbase, task::Task};

pub struct Worker {
    tasks: Vec<Box<dyn Task>>,
    status: Statusbase,
    len: usize,
}
pub trait Workertrait {
    fn get_len(&self) -> usize;
    fn run(&mut self, task: usize);
    fn set_status(&mut self, status: Statusbase);
    fn status(&self) -> Statusbase;
}
impl Worker {
    pub fn new(tasks: Vec<Box<dyn Task>>) -> Worker {
        let mut worker = Worker {
            tasks,
            status: Statusbase::Waiting,
            len: 0,
        };
        worker.len = worker.tasks.len();
        worker
    }
    pub fn get_len(&self) -> usize {
        self.len
    }
    pub fn run(&mut self, i: usize) {
        let task = &mut self.tasks[i];
        match task.status() {
            Statusbase::Ready => {
                task.run();
            }
            Statusbase::Waiting | Statusbase::Running => {}
            Statusbase::Done => {
                self.tasks.remove(i);
            }
            Statusbase::Error(e) => {
                self.status = Statusbase::Error(e.clone());
                self.tasks.remove(i);
                println!("Error: {}", e);
            }
        }
    }
    pub fn set_status(&mut self, _status: Statusbase) {}
    pub fn status(&self) -> Statusbase {
        self.status.clone()
    }
}
