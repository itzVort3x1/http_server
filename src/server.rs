use std::net::TcpListener;

pub struct Server {
     addr: String,
}

impl Server {
     pub fn new(addr: &str) -> Self {
          Self {
               addr: addr.to_string(),
          }
     }

     pub fn run(self) {
          println!("Listening on port: {}", self.addr);
          let listener = TcpListener::bind(&self.addr).unwrap();
     }
}
