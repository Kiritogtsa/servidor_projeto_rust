use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

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
use crate::model::{body::Body, head::Head, request::Request};
pub struct Listener {}
impl Listener {
    // vou usar uma thread do rust para essa parte do ouvinte e passar um channel de comunicação
    // para as requests usando movendo isso para dentro da thread
    fn listener(addr: String) {
        let listener = TcpListener::bind(addr).unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                let streamli = stream.unwrap();
                handle_connection(streamli);
            }
        });
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
