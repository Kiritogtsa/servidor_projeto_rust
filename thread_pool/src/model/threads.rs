use super::{status::Statusbase, worker::Worker};

pub struct Theadcore {
    worker: Vec<Box<Worker>>,

    status: Statusbase,
}
// uso de try_recv, e separação entre thread e worker depois como uma struct para o work

// so implementar o resto que fica numa boa para seguir para o projeto principal, que seria o
// servidor usando essa lib para eu não precisar criar de ultima hora para aceitar novas conetiçoes

impl Theadcore {
    pub fn new(worker: Box<Worker>) -> Self {
        Theadcore {
            worker: vec![worker],
            status: Statusbase::Running,
        }
    }
    pub fn run(&mut self) {
        for i in (0..self.worker.len()).rev() {
            let work = &mut self.worker[i];
            match work.status() {
                Statusbase::Ready => {
                    self.worker[i].run();
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
        }
        if self.worker.is_empty() {
            self.status = Statusbase::Done;
        } else if !matches!(self.status, Statusbase::Error(_)) {
            self.status = Statusbase::Running;
        }
    }
    pub fn status_pool(&self) -> &Statusbase {
        &self.status
    }
}
