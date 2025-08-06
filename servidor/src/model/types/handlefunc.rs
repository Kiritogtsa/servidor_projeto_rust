use std::{net::TcpStream, sync::Arc};

use crate::model::{enuns::intoresult::Intoresult, head::Head};

pub type myfunchandle = Arc<dyn Fn(&mut TcpStream, Head) + Send + Sync>;
