use super::routers::Routes;

pub struct Server {
    routes: Vec<Routes>,
    thread: thread_pool::prelude::Theadcore,
    ip: String,
}
