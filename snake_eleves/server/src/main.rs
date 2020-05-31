#![allow(dead_code,unused_imports)]
use crate::game::*; 
use crate::snake::*; 
pub mod game;
pub mod snake;
use termion::cursor;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

/*
fn main() {
    let mut game = game::init_game();
    game.play();
    // Place le curseur tout en bas
    write!(game.stdout, "{}", cursor::Goto(1, (game::HEIGHT+2) as u16)).unwrap();
}
*/

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            let mut game = game::init_game(stream);
            game.play();
            // Place le curseur tout en bas
            write!(game.stdout, "{}", cursor::Goto(1, (game::HEIGHT+2) as u16)).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
