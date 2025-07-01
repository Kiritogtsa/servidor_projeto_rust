use crate::model::{enuns::intoresult::Intoresult, routers::Args};

pub type myfunchandle = Box<dyn Fn(Option<Vec<Args>>) -> Intoresult + Send + Sync>;
