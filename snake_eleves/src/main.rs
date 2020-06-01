#![allow(dead_code,unused_imports)]
use crate::game::*; 
use crate::snake::*; 
pub mod game;
pub mod snake;
use termion::cursor;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::io;


fn main() {
    let mut input = String::new();
    println!("How many players do you want?:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("All right then, let's play!");
        }
        Err(e) => {
            println!("Sorry, something went wrong: {}", e);
        }
    }
    let mut game = game::init_game();
    game.play();
    // Place le curseur tout en bas
    write!(game.stdout, "{}", cursor::Goto(1, (game::HEIGHT+2) as u16)).unwrap();
}