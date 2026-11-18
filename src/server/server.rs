use crate::server::http_constants::requests::*;
use log::error;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct Server {
    addr: String,
    port: u16,
}

impl Server {
    pub fn new(addr: String, port: u16) -> Self {
        Server { addr, port }
    }

    pub async fn run(&self) {
        Server::listen(self).await;
    }

    async fn listen(&self) {
        let addr: String = format!("{}:{}", self.addr, self.port);
        let tcp_listener = TcpListener::bind(&addr).await;
        match tcp_listener {
            Err(e) => error!("Error while creating tcp listener: {:?}", e),
            Ok(tcp_listener) => {
                log::info!("Server started on: {}", addr);
                socket_dispatcher(tcp_listener).await
            }
        }
    }
}

async fn socket_dispatcher(tcp: TcpListener) {
    loop {
        match tcp.accept().await {
            Err(e) => error!("Error while accepting tcp connection: {:?}", e),
            Ok((stream, _)) => {
                tokio::spawn(handle_raw_connection(stream));
            }
        }
    }
}

async fn handle_raw_connection(mut stream: TcpStream) {
    log::debug!("Accepted connection from {:?}", stream.peer_addr().unwrap());
    let option_request = build_request(&mut stream).await;
    if option_request.is_none() {
        log::debug!("Failed to read request from client, closing connection");
        return;
    }
    let request = option_request.unwrap();
}

async fn build_request(stream: &mut TcpStream) -> Option<Request> {
    let mut raw_request = "".to_string();
    let res = stream.read_to_string(&mut raw_request).await;

    if res.is_err() {
        log::error!("Failed to read header");
        return None;
    }

    let mut raw_sliced = raw_request.split("\r\n\r\n");
    let raw_hadder = raw_sliced.next().unwrap().trim_end();
    let raw_body = raw_sliced.next().unwrap().trim();

    let header = RequestHeader::new_from_bytes(raw_hadder);
    if header.is_err() {
        log::error!("Failed to parse header");
        return None;
    }
    let requset = Request::new(header.unwrap(), raw_body.to_string());
    Some(requset)
}
