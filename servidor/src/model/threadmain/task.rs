use thread_pool::model::{status::Statusbase, task::Task};

use crate::model::types::{self, methods::Methods};
pub struct TaskStruct {
    handle: types::handlefunc::myfunchandle,
    pub url: String,
    pub method: Methods,
    status: Statusbase,
}
impl TaskStruct {
    pub fn status(&self) -> Statusbase {
        self.status.clone()
    }
}

impl Task for TaskStruct {
    // aqui vai vim o head e o tcpstream para o usuario mesmo ter todo o controle da volta
    // aqui tambem vai ser usando o channel para a comunicação sem interroper o fluxo normal do
    // programa
    // isso que dizer que eu vou implementar a thread background que vai enviar a coneção junto com
    // o request junto com o &tcpstream, e isso vai ser modificado no servidor para virar um head,
    // dai vai vim para as tasks com o &tcpstream e o head para o a função so para eu não ter que
    // me estender muito nesse projeto, dai eu vou imitar o go para os handles, e se o usuario
    // quiser um estado ele vai ter que implementar como uma estrutura assim como e feito no go
    // para ele usar a mesma coneção com o banco de dados, ja que o rust não deixa existir uma
    // variavel global
    fn run(&mut self) -> Statusbase {
        todo!()
    }
    fn status(&self) -> Statusbase {
        todo!()
    }
    fn stop(&mut self) {}
}
