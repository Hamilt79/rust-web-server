mod listener;
mod response;
mod router;
mod header;
use listener::Listener;
use response::Response;
use router::Router;
use header::Header;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

fn main() {
    let mut router = Router::new(&"localhost".to_string(), 8080);
    router.add_route();
}
