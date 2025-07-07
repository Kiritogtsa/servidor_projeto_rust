use thread_pool::{
    model::{status::Statusbase, threadcore},
    prelude::Thread,
};

use super::{routers::Route, types::handlefunc::myfunchandle};
pub struct Server {
    routes: Vec<Route>,
    threads: Vec<Box<dyn Thread<Statusbase>>>,
    ip: String,
}
impl Server {
    pub fn post(&mut self, handle: myfunchandle) {}

    pub fn new(ip: String) -> Self {
        let threadcore = Box::new(threadcore::Theadcore::new());
        // depois fazer a outra thread que vai ouvir e essa thread que vai ouvir vai retornar uma
        // requesição e não um statusbase
        Server {
            routes: Vec::new(),
            threads: vec![threadcore],
            ip,
        }
    }
}
