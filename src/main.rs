use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    os::windows::io::AsRawSocket,
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
    let http_request: Vec<_> = buf_reader
        //maps the output into lines line by line, returns <result,error> so we use the unwrap to get each line as a string.
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    //this is the response sent fromt the server to the browser.
    //http-version status-code status-message crlf then headers clf and then message body.

    //Resonse consists of 3 parts. defining them :
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("src/hello2.html").unwrap();
    //important to tell the client what length to expect.
    let content_length = contents.len();

    //formatting the response
    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}");

    //sending the respnse to the browser, converting response to bytes required for network transmission.
    //unwrap ensure connection success.
    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {http_request:#?}");
}
