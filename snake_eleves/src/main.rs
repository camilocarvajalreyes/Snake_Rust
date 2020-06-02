#![allow(dead_code,unused_imports)]
#![allow(unused_mut)]
use crate::game::*; 
use crate::snake::*; 
pub mod game;
pub mod snake;
use termion::cursor;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::io;
use std::thread::sleep;
use std::time::Duration;
const PAUSE: u64 = 2000;

fn instructions(input: String) -> i8 {
    /* Instructions receives the user input and delivers the instructions
    It also shows an error message if the input is invalid*/
    let mut successful: i8 = -1;
    if input.chars().nth(0)==Some('1') {
        println!("You have chosen single mode! Life is a competition not with others, but with ourselves...");
        println!("Move your snake with: i for going up, k for going down, j left and l right!");
        sleep(Duration::from_millis(PAUSE));
        successful = 1;
    } else if input.chars().nth(0)==Some('2'){ 
        println!("Okay for two players! No wise combatant underestimates their antagonist...");
        println!("Player 1: move your snake with i for going up, k for going down, j left and l right!");
        sleep(Duration::from_millis(PAUSE));
        println!("Player 2: move your snake with f for going up, c for going down, x left and v right!");
        sleep(Duration::from_millis(PAUSE));
        successful = 2;
    } else if input.chars().nth(0)==Some('3'){ 
        println!("Okay for three players! No wise combatant underestimates their antagonist...");
        println!("Player 1: move your snake with i for going up, k for going down, j left and l right!");
        sleep(Duration::from_millis(PAUSE));
        println!("Player 2: move your snake with f for going up, c for going down, x left and v right!");
        sleep(Duration::from_millis(PAUSE));
        println!("Player 3: move your snake with h for going up, n for going down, b left and m right!");
        sleep(Duration::from_millis(PAUSE));
        successful = 3;
    } else if input.chars().nth(0)==Some('4'){ 
        println!("Okay for three players! No wise combatant underestimates their antagonist...");
        println!("Player 1: move your snake with i for going up, k for going down, j left and l right!");
        sleep(Duration::from_millis(PAUSE));
        println!("Player 2: move your snake with f for going up, c for going down, x left and v right!");
        sleep(Duration::from_millis(PAUSE));
        println!("Player 3: move your snake with h for going up, n for going down, b left and m right!");
        sleep(Duration::from_millis(PAUSE));
        println!("Player 4: move your snake with w for going up, s for going down, a left and d right!");
        sleep(Duration::from_millis(PAUSE));
        successful = 4;
    } else {
        println!("Pardon? Pick a number between 0 and 4");
    }
    successful
}

fn main() {
    //source : https://www.youtube.com/watch?v=07pDD0uLjYc
    let mut input = String::new();
    println!("How many players do you want? (4 maximum):");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let success = instructions(input);
            if success != -1 {
                println!("All right then, let's play! Press q if you want to exit the game");
                sleep(Duration::from_millis(PAUSE));
                let mut game = game::init_game(success);
                game.play(); ///////////////////////////////////////////////////////////////////////////////
                // Place le curseur tout en bas
                write!(game.stdout, "{}", cursor::Goto(1, (game::HEIGHT+2) as u16)).unwrap();
            }
            else {println!("Try again");}
        }
        Err(e) => {
            println!("Sorry, something went wrong: {}", e);
        }
    }
    /*let mut game = game::init_game();
    game.play();
    // Place le curseur tout en bas
    write!(game.stdout, "{}", cursor::Goto(1, (game::HEIGHT+2) as u16)).unwrap();*/
}