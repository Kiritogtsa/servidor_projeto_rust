use super::{status::Statusbase, worker::Workertrait};

pub struct Theadcore {
    worker: Vec<Box<dyn Workertrait>>,

    status: Statusbase,
}
// uso de try_recv, e separação entre thread e worker depois como uma struct para o work

// so implementar o resto que fica numa boa para seguir para o projeto principal, que seria o
// servidor usando essa lib para eu não precisar criar de ultima hora para aceitar novas conetiçoes

pub trait Thread<T> {
    fn newtask(&mut self, worker: Box<dyn Workertrait>);
    fn run(&mut self) -> Option<T>;
    fn status_pool(&self) -> &Statusbase;
}
impl Theadcore {
    pub fn new() -> Self {
        Theadcore {
            worker: Vec::new(),
            status: Statusbase::Running,
        }
    }
}
impl Thread<Statusbase> for Theadcore {
    fn newtask(&mut self, worker: Box<dyn Workertrait>) {
        self.worker.push(worker);
    }
    fn run(&mut self) -> Option<Statusbase> {
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
    fn status_pool(&self) -> &Statusbase {
        &self.status
    }
}
