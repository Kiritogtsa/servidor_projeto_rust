use std::sync::mpsc::{channel, Receiver, Sender};

use thread_pool::model::{status::Statusbase, task::Task};

// esta parte ta aqui e assim para servir como exemplo de como implementar uma task que não
// interropa o fluxo de exeção do rust
// isso me garante muito poder ainda usando o rust safe
// essa estrutura ta generica ainda porque eu preciso pensar mais tarde quais que vão ser os
// endepoits e ate aonde eu quero ir no servidor e depois eu vou verificar como eu garanto um
// complatilhamento de variaves atraves da minha e worker para as tasks poderem usar
// no caso vai ser so uma variavel chamada de state, para simular um pouco o tokio nesse sentido,
// para eu não criar um nome novo para esse tipo de variaveis
// isso me garante um fluxo de trabalho no servidor por agora
struct taskinteligente<T> {
    reciever: Receiver<String>,
    status: Statusbase,
    object: T,
}
impl taskinteligente<Option<String>> {
    fn new() -> (Self, Sender<String>) {
        let (tx, rx) = channel();
        (
            taskinteligente {
                reciever: rx,
                status: Statusbase::Waiting,
                object: None,
            },
            tx,
        )
    }
}
impl Task for taskinteligente<Option<String>> {
    fn run(&mut self) -> Statusbase {
        match self.status {
            Statusbase::Ready => todo!(),
            Statusbase::Running => todo!(),
            Statusbase::Waiting => match self.reciever.try_recv() {
                Ok(msg) => {
                    self.object = Some(msg.clone());
                    self.status = Statusbase::Ready;
                    Statusbase::Ready
                }
                Err(_) => todo!(),
            },

            Statusbase::Done => todo!(),
            Statusbase::Error(_) => todo!(),
        }
    }

    fn status(&self) -> Statusbase {
        todo!()
    }

    fn stop(&mut self) {
        todo!()
    }
}
fn main() {
    todo!();
}
