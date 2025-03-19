use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::net::TcpStream;
//use crate::Listener;

pub struct Router {
    routes: Vec<fn(u16)>
}

impl Router {
    pub fn new() {
        // let mut server = Listener::new(8080);
       // server.start_listener(|mut stream| { 
       //     let reader = BufReader::new(&stream);
       //     let http_request: Vec<_> = reader.lines().map(|res| { res.unwrap() }).take_while(|line| { !line.is_empty() }).collect();
       //     let mut header = Header::new(None, None, None, None);
       //     header.fill_from_vec(&http_request);
       //     println!("Methd: {} Path: {}", header.method, header.path);
       //     for (key, val) in header.headers {
       //         println!("Key: {} Value: {}", key, val);
       //     }
       //     let res = Response::new("Woahhhh".to_owned(), 200, None);
       //     let write_res = stream.write_all(res.to_string().as_bytes());
       //     match write_res {
       //         Ok(_)=> (),
       //         Err(e)=>println!("Failed to write to stream. {e}"),
       //     }
       //     println!("Req: {http_request:#?}");
       // });
    }

    pub fn add_route(func: fn(TcpStream)) {
        todo!();
    }
}
