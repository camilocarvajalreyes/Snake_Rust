extern crate rand;

use crate::snake::*;
use std::io::{stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, clear, color, cursor, AsyncReader};
use rand::Rng;
use std::io::Read;
use std::io;

// Largeur du terrain
pub const WIDTH: usize = 60;
// Longueur du terrain
pub const HEIGHT: usize = 20;
// First valid X coordinate in the field
pub const FIRST_X: usize = 0;
// First valid Y coordinate in the field
pub const FIRST_Y: usize = 0;

// Caractère représentant une pomme
const FOOD_CHAR: char = 'Ծ';
// Caractère du serpent
const SNAKE_CHAR: char = '*';
// Le temps que prend 1 tour de jeu en millisecondes
const SPEED: u64 = 1000;

pub struct Game {
    // Sortie stdout en mode "raw"
    pub stdout: RawTerminal<Stdout>,
    // Une entrée stdin asynchrone
    stdin: AsyncReader,
    snake: Snake,
    food: Point,
    speed: u64,
    field: [[char; WIDTH]; HEIGHT],
}

// Une coordonnée de notre terrain
pub struct Point {
    pub x: u16,
    pub y: u16,
}

// Une strcture de direction haut niveau
#[derive(PartialEq, Copy, Clone)]
pub enum Dir {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }
    pub fn go(&self, direction: Dir) -> Point { //returns another point moved in the specific direction, maybe change name
        //direction 0 if up, 1 left, 2, down, 3 right
        let mut xcor = self.x;
        let mut ycor = self.y;
        if direction == Dir::UP { //move up
            ycor = ycor +1;
        } else if direction == Dir::LEFT { //move left
            xcor = xcor - 1;
        } else if direction == Dir::DOWN { //moving down
            ycor = ycor - 1;
        } else if direction == Dir::RIGHT { //move right
            xcor = xcor + 1;
        }
        Point::new(xcor, ycor) //we must verify if it works!
    }
}

impl Game {
    // Dessine les bordures du terrain de jeu
    pub fn draw_field(&mut self) {
        // On écrit dans notre console statique dans l'ordre
        // - on efface tout le contenu
        // - place le curseur au début de la première ligne
        // - la couleur du ForeGround choisie est bleu
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            color::Fg(color::Blue)
        )
        .unwrap();
        // On appelle flush() pour forcer les modifications dans
        // stdout
        self.stdout.flush().unwrap();

        // Affichage de l'espace de jeu
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                write!(self.stdout, "{}", self.field[i][j]).unwrap();
            }
            // Passe à la ligne suivante et replace le curseur en début de ligne
            write!(self.stdout, "{}\n", cursor::Goto(1, (i + 1) as u16)).unwrap();
        }

        // Remet à jour la couleur utilisé
        write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap();
        self.stdout.flush().unwrap();
    }

    // Dessine une pomme aléatoirement dans le terrain de jeu
    pub fn draw_food(&mut self) {
        // 4 étapes
        // - place le curseur à la position souhaitée
        // - choisit une couleur pour la pomme
        // - écrit le caractère correspondant à la pomme
        // - remet à zéro la couleur pour les prochaines utilisations
        write!(
            self.stdout,
            "{}{}{}{}",
            cursor::Goto(self.food.x, self.food.y),
            color::Fg(color::Red),
            FOOD_CHAR,
            color::Fg(color::Reset)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn play(&mut self) {
        let mut i = 1;
        let mut command;
        let mut command_on = true;

        loop {
            write!(self.stdout, "{}", cursor::Hide).unwrap();
            self.draw_snake();
            i += 1;
            // Pause le programme pendant _self.speed_
            sleep(Duration::from_millis(self.speed));
            command = try_command();
            if command_on {
                if command == "quit" {
                    self.stdout.flush().unwrap(); //maybe should be different
                    break
                }
                else if command == "grow" {
                    self.snake.grow();
                }
                else if command == "deactivate" {
                    command_on = false;
                }
            }
        }
    }

    pub fn draw_snake(&mut self) {
        for p in self.snake.body.iter() {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(p.x, p.y),
                SNAKE_CHAR
            )
            .unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

fn try_command() -> String {
    let input = getch();

    if input == 'q' {
        return String::from("quit")
    }
    else if input == 'g' {
        return String::from("grow")
    }
    else if input == 'd' {
        return String::from("deactivate")
    }

    String::from("")
}

// Génère aléatoirement un point dans l'espace du jeu
// où sera placé la prochaine pomme
fn generate_food() -> Point {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range((FIRST_X+1) as u16, (WIDTH) as u16);
    let y = rng.gen_range((FIRST_Y+1) as u16, (HEIGHT-1) as u16);
    Point::new(x, y)
}

// Initialise un espace de jeu
// Dessine les bords de l'espace et met un caractère vide pour le reste
pub fn init_field() -> [[char; WIDTH]; HEIGHT] {
    //TODO
    let mut field = [[' '; WIDTH]; HEIGHT];
    let c = '#';
    for i in 0..(WIDTH) {
        field[FIRST_Y][i] = c;
        field[HEIGHT -1][i] = c;
    }
    for j in 0..HEIGHT {
        field[j][FIRST_X] = c;
        field[j][WIDTH -1] = c;
    }

    field
}

// Initialise une structure Game
pub fn init_game() -> Game {
    // Donne une console "statique" permettant de faire des
    // applications dans le terminal
    // Voir la documentation de _termion__ pour plus d'informations
    let stdout = stdout().into_raw_mode().unwrap();
    let stdin = async_stdin();
    let initial_point = Point::new(5, 5);
    initnscr();
    raw();

    let game = Game {
        stdout: stdout,
        stdin: stdin,
        snake: Snake::new(initial_point),
        food: generate_food(),
        speed: SPEED,
        field: init_field(),
    };

    game
}

//NEXT Try to do non blocking input