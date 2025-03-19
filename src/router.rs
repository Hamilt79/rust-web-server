use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::net::TcpStream;
use crate::listener::Listener;
use crate::header::Header;

pub struct Router {
    routes: Vec<fn(u16)>
}

impl Router {
    pub fn new(address: &String, port: u16) -> Router {
        let mut server = Listener::new(port, address);
        let router = Router { routes: Vec::new() };
        server.start_listener(|mut stream| { 
            let reader = BufReader::new(&stream);
            let http_request: Vec<_> = reader.lines().map(|res| { res.unwrap() }).take_while(|line| { !line.is_empty() }).collect();
            let mut header = Header::new(None, None, None, None);
            header.fill_from_vec(&http_request);
            for func in router.routes {
                
            }
            match write_res {
                Ok(_)=> (),
                Err(e)=>println!("Failed to write to stream. {e}"),
            }
        });
        router
    }

    pub fn add_route(func: fn(TcpStream)) {
        //todo!();
    }
}
