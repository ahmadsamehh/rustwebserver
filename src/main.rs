use std::{
    fmt::format,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

//handling the request sent from the browser to our server.
//it keeps reading the lines line by line untill it gets two empty lines then the request is done.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    //gets the first line in the http request .lines().next() then we use 2 unwraps 1 for each method of these.
    let request_line_rustbook = buf_reader.lines().next().unwrap().unwrap();
    //this print is for debugging purpose.
    println!("request-line = {}", request_line_rustbook);

    //another way to do it.
    // let request_line = &http_request[0];

    let (status_line, uri) = if request_line_rustbook == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src/hello2.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let content = fs::read_to_string(uri).unwrap();
    let content_length = content.len();

    let response_formatted =
        format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
    stream.write_all(response_formatted.as_bytes()).unwrap();

    // if request_line_rustbook == "GET / HTTP/1.1" {
    //     //sending the respnse to the browser, converting response to bytes required for network transmission.
    //     //unwrap ensure connection success.
    //     stream
    //         .write_all(send_response("src/hello2.html", "HTTP/1.1 200 OK").as_bytes())
    //         .unwrap();
    // } else {
    //     stream
    //         .write_all(send_response("src/404.html", "HTTP/1.1 404 NOT FOUND").as_bytes())
    //         .unwrap();
    // }
}
fn send_response(uri: &str, status_line: &str) -> String {
    //this is the response sent fromt the server to the browser, http-version status-code status-message crlf then headers clf and then message body.
    //Resonse consists of 3 parts. defining them :
    let contents = fs::read_to_string(uri).unwrap();

    //important to tell the client what length to expect.
    let content_length = contents.len();

    //formatting the response
    return format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}");
}
