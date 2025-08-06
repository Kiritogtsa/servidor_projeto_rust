mod model;
use model::body::*;
use model::head::*;
use model::request::Request;

use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{channel, Receiver, Sender},
};
use thread_pool::model::{status::Statusbase, task::Task};

// usar canais para separar a parte do servidor e do ouvinte de coneçoes em threads, e a
// cominicação deve ser feita atraves dos canais, e o uso de try_recv e Receiver e essencial, para
// não afetar os routes/workes para isso garantir uma boa separação de atividades, tambem usar o
// worker para manter a rota, e com isso ele mantem o handle function, metodo e o path("/(alguma
// coisa)"), isso me garante que quando eu for criar novas task eu passe a função correta para ela
// e ela executa em forma de try_recv tambem para me da os status

// esta parte ta aqui e assim para servir como exemplo de como implementar uma task que não
// interropa o fluxo de exeção do rust
// isso me garante muito poder ainda usando o rust safe
// essa estrutura ta generica ainda porque eu preciso pensar mais tarde quais que vão ser os
// endepoits e ate aonde eu quero ir no servidor e depois eu vou verificar como eu garanto um
// complatilhamento de variaves atraves da minha e worker para as tasks poderem usar
// no caso vai ser so uma variavel chamada de state, para simular um pouco o tokio nesse sentido,
// para eu não criar um nome novo para esse tipo de variaveis
// isso me garante um fluxo de trabalho no servidor por agora
// struct taskinteligente<T> {
//     reciever: Receiver<String>,
//     status: Statusbase,
//     object: T,
// }
// impl taskinteligente<Option<String>> {
//     fn new() -> (Self, Sender<String>) {
//         let (tx, rx) = channel();
//         (
//             taskinteligente {
//                 reciever: rx,
//                 status: Statusbase::Waiting,
//                 object: None,
//             },
//             tx,
//         )
//     }
// }
// impl Task for taskinteligente<Option<String>> {
//     fn run(&mut self) -> Statusbase {
//         match self.status {
//             Statusbase::Ready => todo!(),
//             Statusbase::Running => todo!(),
//             Statusbase::Waiting => match self.reciever.try_recv() {
//                 Ok(msg) => {
//                     self.object = Some(msg.clone());
//                     self.status = Statusbase::Ready;
//                     Statusbase::Ready
//                 }
//                 Err(_) => todo!(),
//             },
//
//             Statusbase::Done => todo!(),
//             Statusbase::Error(_) => todo!(),
//         }
//     }
//
//     fn status(&self) -> Statusbase {
//         todo!()
//     }
//
//     fn stop(&mut self) {
//         todo!()
//     }
// }
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    // Ler as linhas do header até a linha vazia (CRLF)
    let mut header_lines = Vec::new();
    loop {
        let mut line = String::new();
        let bytes = buf_reader.read_line(&mut line).unwrap();
        if bytes == 0 || line.trim().is_empty() {
            break;
        }
        header_lines.push(line.trim_end().to_string());
    }

    // Tenta converter para Head
    let head = match Head::try_from(header_lines.as_slice()) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Erro ao parsear header: {}", e);
            return;
        }
    };

    // Pega Content-Length para ler body
    let content_length = head
        .headers
        .get("content-length")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    // Lê o body
    let mut body_buf = vec![0; content_length];
    if content_length > 0 {
        buf_reader.read_exact(&mut body_buf).unwrap();
    }
    let raw_body = String::from_utf8_lossy(&body_buf).to_string();

    // Monta o enum Body (por enquanto só String)
    let body = Body::String(raw_body);

    // Monta o Request
    let request = Request { head, body };

    // Aqui você pode fazer o que quiser com o request, por exemplo imprimir
    println!("Request head: {:?}", request.head);
    match request.body {
        Body::String(ref s) => println!("Body (string): {}", s),
        Body::Json(_) => println!("Body (json)"),
    }

    // Para responder, você pode seguir seu exemplo:
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("servidor/src/pages/index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write(response.as_bytes()).unwrap();
}
