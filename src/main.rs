use std::io::{ BufRead, Write };

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut rdr = std::io::BufReader::new(&mut stream);
    let mut l = String::new();
    rdr.read_line(&mut l).unwrap();

    match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
        ["GET", resource, "HTTP/1.1"] => {
            /*Printing the whole request:
            loop {
                rdr.read_line(&mut l).unwrap();
                if l.trim().is_empty() { 
                    break;
                }
                println!("{l}");
            }*/
            let mut p = std::path::PathBuf::new();
            p.push("public");
            p.push(resource);
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n<h1>Hello, World!</h1>").unwrap();
            //stream.write_all(&std::fs::read(p).unwrap()).unwrap();
        }
        _ => todo!()
    }
}
fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("Connection failed! {e}");
            }
        }
    }
}
