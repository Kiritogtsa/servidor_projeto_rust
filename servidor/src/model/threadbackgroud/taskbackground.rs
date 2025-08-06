use std::sync::mpsc::Receiver;

use thread_pool::model::status::Statusbase;

use crate::model::request::Request;
pub struct Taskbrackground {
    reciver: Receiver<Request>,
    // criar um tipo para uma funçaõ
    // background: Box<dyn Fn>,
}
