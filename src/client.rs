use std::net::TcpStream;
use std::io::prelude::*;
use std::io::{Read,Write};

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8080")
                       .expect("Couldn't connect to the server...");
    let message = "Hello, server!";
    stream.write(message.as_bytes()).expect("Failed to write to stream");
                
   // stream.read(&mut [0; 128])?;
     // Buffer to read the server's response
     let mut buffer = [0; 128];
     let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
 
     // Convert the response to a string and print it
     let response = String::from_utf8_lossy(&buffer[..bytes_read]);
     println!("Received from server: {}", response);

        loop {
            println!("Enter a message: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Could not read from stdin");
            //write the input ot the server
            input = input.trim().to_string();
            println!("{}",input);
            stream.write(input.as_bytes()).expect("Could not pass the input");

        
            // Read the server's response into the buffer
            let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
 
            // Convert the response to a string and print it
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Received from server: {}", response);
            }
      

}