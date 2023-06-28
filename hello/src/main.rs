use hello::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // add the take method with the limit num of requests as its arg so a graceful shutdown can be fulfilled when the num of requests reaches it
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down...");
    // let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for stream in listener.incoming() {
    //     let stream: TcpStream = stream.unwrap();

    //     handle_connection(stream);
    // }
}

fn handle_connection(mut stream: TcpStream) {
    /* Multi-threaded server pattern with a thread pool */
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    /* Simple single-threaded server pattern with conditional response */
    // let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();

    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    // let contents = fs::read_to_string(filename).unwrap();
    // let length = contents.len();

    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap();

    /* Simple single-threaded server pattern with unconditional response */
    // let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    // // define req
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // // res format
    // // HTTP-Version Status-Code Reason-Phrase CRLF
    // // headers CRLF
    // // message-body
    // let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("hello.html").unwrap();
    // let length = contents.len();

    // // define res
    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap(); // need to convert the string data to bytes
    // println!("Request: {:#?}", http_request);
}
