use std::net::{TcpListener, TcpStream};

pub struct Listener {
    port: i16,
    listener: Option<TcpListener>,
}

impl Listener {

    pub fn new(port: i16) -> Listener {
        Listener { port, listener: None }
    }

    pub fn start_listener(&mut self, f: fn(TcpStream)) {
        let listener = TcpListener::bind("127.0.0.1:".to_owned() + &self.port.to_string());
        match listener {
            Ok(temp) => { self.listener = Some(temp); },
            Err(error) => { panic!("Could not open TCP port. {error}"); }
        }
        match &self.listener {
            Some(temp) => { 
                for stream in temp.incoming() {
                    match stream {
                        Ok(stream) => { f(stream) },
                        Err(e) => { println!("Connection Failed. {e}") }
                    }
                }
            },
            None => { panic!("No Listener"); }
        }
    }
}
