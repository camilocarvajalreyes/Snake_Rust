use crate::snake::*;
use std::io::{stdout, Stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, clear, color, cursor, AsyncReader};

// Largeur du terrain
pub const WIDTH: usize = 60;
// Longueur du terrain
pub const HEIGHT: usize = 20;
// Caractère représentant une pomme
const FOOD_CHAR: char = 'Ծ';
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

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }
    pub fn move(&self, direction : i32) -> Point { //returns another point moved in the specific direction
        //direction 0 if up, 1 left, 2, down, 3 right
        let mut xcor = self.x;
        let mut ycor = self.y;
        if direction==0 { //move up
            ycor = ycor +1;
        } else if direction==1 { //move left
            xcor = xcor - 1;
        } else if direction==2 { //moving down
            ycor = ycor - 1;
        } else if direction==3 { //move right
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
        loop {
            write!(self.stdout, "{}", cursor::Goto(1, HEIGHT as u16 + i)).unwrap();
            println!("Encore un tour de jeu");
            i += 1;
            // Pause le programme pendant _self.speed_
            sleep(Duration::from_millis(self.speed));
        }
    }
}

// Génère aléatoirement un point dans l'espace du jeu
// où sera placé la prochaine pomme
fn generate_food() -> Point {
    //TODO
    Point::new(10, 10)
}

// Initialise un espace de jeu
// Dessine les bords de l'espace et met un caractère vide pour le reste
pub fn init_field() -> [[char; WIDTH]; HEIGHT] {
    //TODO
    let mut field = [[' '; WIDTH]; HEIGHT];
    let c = '#';
    for i in 0..(WIDTH) {
        field[HEIGHT / 2][i] = c;
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

    let game = Game {
        stdout: stdout,
        stdin: stdin,
        snake: Snake {}, // to do
        food: generate_food(),
        speed: SPEED,
        field: init_field(),
    };
    game
}
