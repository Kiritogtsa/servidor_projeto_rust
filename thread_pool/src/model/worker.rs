use super::{status::Statusbase, task::Task};

pub struct Worker {
    tasks: Vec<Box<dyn Task>>,
    status: Statusbase,
}

impl Worker {
    pub fn new(tasks: Vec<Box<dyn Task>>) -> Worker {
        Worker {
            tasks,
            status: Statusbase::Waiting,
        }
    }
    pub fn run(&mut self) {
        let mut i = 0;
        while i < self.tasks.len() {
            let task = &mut self.tasks[i];
            match task.status() {
                Statusbase::Ready => {
                    task.run();
                    i += 1;
                }
                Statusbase::Waiting | Statusbase::Running => {
                    i += 1;
                }
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
    }
    pub fn status(&self) -> &Statusbase {
        &self.status
    }
}
