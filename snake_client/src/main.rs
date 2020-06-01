use std::net::{TcpStream};
use std::io::{Read, Write};
//use std::str::from_utf8;
use termion::raw::{IntoRawMode};
use std::io::{stdout};
use termion::{async_stdin, AsyncReader};

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let mut stdin = async_stdin();
            //let quit_msg = b"p";
            //let mut stdout = stdout().into_raw_mode().unwrap();
            let mut buffer = String::from("hi");
            let mut playing = true;
            while playing {
                // get user input
                //stdin.read_to_string(&mut buffer).expect("Problem reading user input!");
                // write to tcp
                let msg = b"Hello!";

                stream.write(msg).unwrap();
                println!("Sent Hello, awaiting reply...");
    
                let mut data = [0 as u8; 6]; // using 6 byte buffer
                
                println!("before reading from server {}", buffer);
                //test_stream = AsyncReader::new();
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        println!("Reply is ok!");
                        
                        write!(std::io::stdout(),"{}\n", String::from_utf8((&data).to_vec()).unwrap()).unwrap();
                        
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
                playing = false;
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}