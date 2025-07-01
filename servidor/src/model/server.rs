use thread_pool::model::threads;

use super::{routers::Route, types::handlefunc::myfunchandle};
pub struct Server {
    routes: Vec<Route>,
    thread: thread_pool::prelude::Theadcore,
    ip: String,
}
impl Server {
    pub fn post(&mut self, handle: myfunchandle) {}

    pub fn new(ip: String) -> Self {
        Server {
            routes: Vec::new(),
            thread: threads::Theadcore::new(),
            ip,
        }
    }
}
