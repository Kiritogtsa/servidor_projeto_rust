use super::{
    routers::Route, threadbackgroud::threadbackground::Threadbackground,
    threadmain::threadcore::Theadcore, types::handlefunc::myfunchandle,
};
pub struct Server {
    routes: Vec<Route>,
    thread: Theadcore,
    threadbackground: Threadbackground,
    ip: String,
}
impl Server {
    pub fn post(&mut self, handle: myfunchandle) {}
    pub fn get(&mut self, handle: myfunchandle) {}
    pub fn new(ip: String) -> Self {
        // depois fazer a outra thread que vai ouvir e essa thread que vai ouvir vai retornar uma
        // requesição e não um statusbase
        Server {
            routes: Vec::new(),
            thread: Theadcore::new(),
            threadbackground: Threadbackground::new(),
            ip,
        }
    }
    pub fn run(&mut self) {
        self.thread.newworker(self.routes.clone());
        loop {
            // essa seria a minha primeira thread com os worker/rotas
            // dai teria a segunda thread sendo um ouvinte e passado o tcpstream junto com a
            // requisição via canal como tupla
            self.thread.run();
        }
    }
}
