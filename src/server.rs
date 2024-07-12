use std::{io::Read, io::Write, net::TcpListener};
use crate::http::{request, response, ParseError, Request, Response, StatusCode};  


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
    
    pub struct Server {
        addr: String
     }
     
     impl Server {
        pub fn new (addr: String) -> Self {
             Server {
                 addr
             }
         }   
         
        pub fn run (self, mut handler: impl Handler) {
             println!("Listening on {}", self.addr);

            let listener = TcpListener::bind(&self.addr).unwrap();

            loop {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response =  match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                },
                                Err(err) => {
                                    println!("Failed to parse a request: {}", err);
                                    handler.handle_bad_request(&err)
                                }
                               };
                               if let Err(err) = response.send(&mut stream) {
                                println!("Failed to send response: {}", err)
                               }
                             },
                            Err(err) => println!("Failed to read from connection: {}", err),
                        }
                    },
                    Err(err) => {
                        println!("Failed to establish a connection: {}", err)
                    }
            }
         }
     }
 
    }