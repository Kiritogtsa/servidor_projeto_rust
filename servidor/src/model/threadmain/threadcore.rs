use thread_pool::model::status::Statusbase;

use crate::model::{routers::Route, types::methods::Methods};

use super::{
    task::TaskStruct,
    worker::{self, Worker},
};

pub struct Theadcore {
    worker: Vec<Box<Worker>>,
    status: Statusbase,
}
impl Theadcore {
    pub fn new() -> Self {
        Theadcore {
            worker: Vec::new(),
            status: Statusbase::Running,
        }
    }
}

impl Theadcore {
    pub fn newtask(&mut self, task: TaskStruct) {
        for worker in self.worker.iter_mut() {
            match (worker.route.method.clone(), task.method.clone()) {
                (Methods::POST, Methods::POST) => {
                    if worker.route.path.clone() == task.url {
                        worker.addtask(Box::new(task));
                        break;
                    }
                }
                (Methods::GET, Methods::GET) => {
                    if worker.route.path.clone() == task.url {
                        worker.addtask(Box::new(task));
                        break;
                    }
                }
                _ => println!(),
            }
        }
    }
    pub fn run(&mut self) -> Option<Statusbase> {
        for i in (0..self.worker.len()).rev() {
            let work = &mut self.worker[i];
            let len = work.get_len();
            let mut y: usize = 0;
            while len > y {
                // o match com self.worker[i].status cria uma referencia que e mantida ante o fim
                // do bloco do match isso significa que ele pode reclamar de tar modificando as
                // tasks
                // dai o match (&mut self,worker[i].status) ele ta criando uma referencia para o
                // resultando e não para o worker denovo
                match (&mut self.worker[i]).status() {
                    Statusbase::Ready => {
                        self.worker[i].run(y);
                    }
                    Statusbase::Waiting | Statusbase::Running => {}
                    Statusbase::Done => {
                        self.worker.remove(i);
                    }
                    Statusbase::Error(e) => {
                        self.status = Statusbase::Error(e.clone());
                        println!("Error: {}", e);
                        self.worker.remove(i);
                    }
                }
                y += 1;
            }
        }
        if self.worker.is_empty() {
            self.status = Statusbase::Done;
        } else if !matches!(self.status, Statusbase::Error(_)) {
            self.status = Statusbase::Running;
        }
        match self.status {
            Statusbase::Ready => None,
            Statusbase::Running => None,
            Statusbase::Waiting => None,
            Statusbase::Done => Some(Statusbase::Done),
            Statusbase::Error(_) => Some(Statusbase::Error(format!(" erro a thread principal"))),
        }
    }
    pub fn status_pool(&self) -> &Statusbase {
        &self.status
    }

    pub fn newworker(&mut self, routes: Vec<Route>) {
        for route in routes {
            self.worker.push(Box::new(worker::Worker::new(route)));
        }
    }
}
