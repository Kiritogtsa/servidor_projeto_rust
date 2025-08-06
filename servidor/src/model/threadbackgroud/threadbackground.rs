use super::workerbrackground::Workerbackgroung;

pub struct Threadbackground {
    worker: Workerbackgroung,
}
impl Threadbackground {
    pub fn new() -> Self {
        Threadbackground {
            worker: Workerbackgroung::new(),
        }
    }
}
