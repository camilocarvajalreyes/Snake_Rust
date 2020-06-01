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
// First valid X coordinate inside the field
pub const FIRST_X: usize = 2;
// First valid Y coordinate inside the field
pub const FIRST_Y: usize = 2;
// Last valid X coordinate inside the field
pub const LAST_X: usize = WIDTH-1;
// Last valid Y coordinate inside the field
pub const LAST_Y: usize = HEIGHT-1;


// Caractère représentant une pomme
const FOOD_CHAR: char = 'Ծ';
// Caractère du serpent
const SNAKE_CHAR: char = '*';
// Le temps que prend 1 tour de jeu en millisecondes
const SPEED: u64 = 500;

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
#[derive(PartialEq)] 
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
            ycor = ycor - 1;
        } else if direction == Dir::LEFT { //move left
            xcor = xcor - 1;
        } else if direction == Dir::DOWN { //moving down
            ycor = ycor + 1;
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

    /*
    Plays the game with the following set of keys U/D/L/R:
        - U/H/J/K
    */
    pub fn play(&mut self) {
        let mut buffer = String::new();
        //hide cursor
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        loop {
            // sleep according to game speed
            sleep(Duration::from_millis(self.speed));
            //did it collide with a wall ?
            if self.snake_hit_wall() {
                break
            }
            //did it collide with itself ?
            if self.snake_hit_itself() {
                break
            }
            //did it reach the food ?
            if self.snake_got_food() {
                self.snake.grow();
                self.food = generate_food();
            }
            //draw elements
            self.draw_field();
            self.draw_food();
            self.draw_snake();
            //asynchronous read
            self.stdin.read_to_string(&mut buffer).expect("");
            //println!("buffer = {}", buffer); 
            //treat input
            if buffer == "q" {
                self.stdout.flush().unwrap(); //maybe should be different
                break
            }
            else if buffer.contains("l") {
                self.snake.turn(Dir::RIGHT);
            }
            else if buffer.contains("i") {
                self.snake.turn(Dir::UP);
            }
            else if buffer.contains("k") {
                self.snake.turn(Dir::DOWN);
            }
            else if buffer.contains("j") {
                self.snake.turn(Dir::LEFT);
            }
            else if buffer == "w" {
                self.snake.grow();
                self.snake.grow();
                self.snake.grow();
                self.snake.grow();
                self.snake.grow();
            }
            //moves the snake
            self.snake.go_forward();
            //reset buffer
            buffer = String::from("");
        }
        //unhide cursor
        write!(self.stdout, "{}", cursor::Show).unwrap();
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

    /*
    check collision between the snake and the walls
    returns true if it hit a wall or false otherwise
    NEED TO IMPLEMENT COLLISION OF THE SNAKE WITH ITSELF
    */
    fn snake_hit_wall(&self) -> bool {
        let head = self.snake.body.back().unwrap();
        if head.x > LAST_X as u16 || head.x < FIRST_X as u16 
            || head.y > LAST_Y as u16 || head.y < FIRST_Y as u16
        {
            return true;
        }

        false
    }

    fn snake_hit_itself(&mut self) -> bool {
        let head = self.snake.body.pop_back().unwrap();
        for body_part in self.snake.body.iter() {
            if head == *body_part {
                return true;
            }
        }
        self.snake.body.push_back(head);
        false
    }

    fn snake_got_food(&self) -> bool {
        let head = self.snake.body.back().unwrap();
        if head == &self.food {
            return true;
        }
        false
    }

}

// Génère aléatoirement un point dans l'espace du jeu
// où sera placé la prochaine pomme
fn generate_food() -> Point {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(FIRST_X as u16, LAST_X as u16);
    let y = rng.gen_range(FIRST_Y as u16, LAST_Y as u16);
    Point::new(x, y)
}

// Initialise un espace de jeu
// Dessine les bords de l'espace et met un caractère vide pour le reste
pub fn init_field() -> [[char; WIDTH]; HEIGHT] {
    //TODO
    let mut field = [[' '; WIDTH]; HEIGHT];
    let c = '#';
    for i in 0..(WIDTH) {
        field[FIRST_Y -2][i] = c;
        field[HEIGHT -1][i] = c;
    }
    for j in 0..HEIGHT {
        field[j][FIRST_X -2] = c;
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