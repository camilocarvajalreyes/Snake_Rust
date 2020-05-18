#![allow(dead_code)]
pub mod game;
pub mod snake;
use std::io::Write;
use termion::cursor;

fn main() {
    let mut game = game::init_game();
    game.draw_field();
    game.draw_food();
    game.play();
    // Place le curseur tout en bas
    write!(game.stdout, "{}", cursor::Goto(1, game::HEIGHT as u16)).unwrap();
}
