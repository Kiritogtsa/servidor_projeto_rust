use crate::model::routers::Route;
use thread_pool::model::status::Statusbase;
use thread_pool::model::task::Task;

use super::task::TaskStruct;

pub struct Worker {
    tasks: Vec<Box<TaskStruct>>,
    status: Statusbase,
    len: usize,
    pub route: Route,
}
impl Worker {
    pub fn new(route: Route) -> Worker {
        let mut worker = Worker {
            tasks: Vec::new(),
            status: Statusbase::Waiting,
            len: 0,
            route,
        };
        worker.len = worker.tasks.len();
        worker
    }
    pub fn get_len(&self) -> usize {
        self.len
    }
    pub fn addtask(&mut self, task: Box<TaskStruct>) {
        self.tasks.push(task);
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
