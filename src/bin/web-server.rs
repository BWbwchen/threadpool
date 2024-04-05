use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

const MAX_THREAD_NUM: usize = 4;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(MAX_THREAD_NUM);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_req = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!("Request: {:?}", http_req);

    let resp = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(resp.as_bytes()).unwrap();
}
