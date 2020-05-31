#![allow(dead_code,unused_imports)]
use crate::game::*; 
use crate::snake::*; 
pub mod game;
pub mod snake;
use std::io::Write;
use termion::cursor;

fn main() {
    let mut game = game::init_game();
    game.play();
    // Place le curseur tout en bas
    write!(game.stdout, "{}", cursor::Goto(1, (game::HEIGHT+2) as u16)).unwrap();
}
