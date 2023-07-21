//importing 
use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
use std::thread::spawn;
// use std::String;

fn handle_client(mut stream: TcpStream){//stream is the client connection
    //read data from client 
    let mut buffer =[0;128];
    loop {
        match stream.read(&mut buffer){
            Ok(0) =>{ //0 bytes were read
                break;
            }
            Ok(_) => { //some bytes were read
                let request = String::from_utf8_lossy(&buffer[..]);
                println!("Received request: {}", request);
                if request == "exit"{
                    break;
            }
            let response ="Hello,client!".as_bytes();
            stream.write(response).expect("cant write a response");

            }
            Err(e) => { //could not read
                eprintln!("Client's stream cant be read")
            }
        }
    }
}

//entry proint
fn main(){
    let socket = "127.0.0.1:8080";
    let listener = TcpListener::bind(socket).expect("Listner failed to bind to a address");
    println!("server listining to {}",socket);
    
    for stream in listener.incoming(){ //incoming iterates over current connections and after that it blocks and waits for new connections . therefore theread spawn is necessary to operate on actual connections
        match stream{
            Ok(stream) => {
                std::thread::spawn(||handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish the connection:{}",e);
            }
        }
    }
    
}
