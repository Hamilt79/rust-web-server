use std::net::{TcpListener, TcpStream};

pub struct Listener {
    port: u16,
    address: String,
    listener: Option<TcpListener>,
}

impl Listener {

    pub fn new(port: u16, address: &String) -> Listener {
        Listener { port, address: address.clone(), listener: None }
    }

    pub fn start_listener(&mut self, f: fn(TcpStream)) {
        let listener = TcpListener::bind(self.address.clone() + ":" + &self.port.to_string());
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
