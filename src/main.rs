use rserver::ThreadPool;
use std::{
    fmt::format,
    fs,
    io::{prelude::*, BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}
fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request_line = reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            "/home/yanxinxiang/code/rust_server/pages/hello.html",
        ),
        "GET  /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            (
                "HTTP/1.1 404 NOT FOUND",
                "/home/yanxinxiang/code/rust_server/pages/404.html",
            )
        }
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            "/home/yanxinxiang/code/rust_server/pages/404.html",
        ),
    };
}
