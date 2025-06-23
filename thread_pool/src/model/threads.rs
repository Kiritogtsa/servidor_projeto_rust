use super::task::Task;

pub struct Theadcore {
    tasks: Vec<Box<dyn Task>>,
}

// so implementar o resto que fica numa boa para seguir para o projeto principal, que seria o
// servidor usando essa lib para eu não precisar criar de ultima hora para aceitar novas conetiçoes

impl Theadcore {
    pub fn new(task: Box<dyn Task>) -> Self {
        Theadcore { tasks: vec![task] }
    }
}
