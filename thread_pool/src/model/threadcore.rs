use super::{status::Statusbase, task::Task, worker::Workertrait};

// uso de try_recv, e separação entre thread e worker depois como uma struct para o work

// so implementar o resto que fica numa boa para seguir para o projeto principal, que seria o
// servidor usando essa lib para eu não precisar criar de ultima hora para aceitar novas conetiçoes

pub trait Thread<T> {
    fn newtask(&mut self, task: Box<dyn Task>);
    fn run(&mut self) -> Option<Statusbase>;
    fn status_pool(&self) -> &Statusbase;
    fn newworker(&mut self, worker: Box<dyn Workertrait<T>>);
}
