// Structure du serpent
use crate::game::*; 
//pub mod game;// importing structures and objects from game.rs (?)
use std::collections::VecDeque;

pub struct Snake { //construction of the strut snake
    pub body: VecDeque<Point>, //body of the snake as a list of Points
    pub direction: Dir, 
}
impl Snake {
    pub fn new(initial_point: Point) -> Self { 
        let mut vec = VecDeque::new();
        vec.push_back(initial_point);
        Snake {body: vec, direction: Dir::RIGHT} //function that returns a single point snake going up as default
    }

    /*
    Moves the snake forward but takes into account the dimension distortion of the field.
    Moves two points horizontally and only one vertically
    */
    pub fn go_forward(&mut self) {
        // Leaving the idea on hold because it caused problems regarding eating food
        /* 
        if self.direction == Dir::RIGHT || self.direction == Dir::LEFT {
            self.forward();
        }
        */
        self.forward();
    }

    /* adds a point in the actual direction to the back (head)
       then deletes the element in the front (tail) 
    */
    fn forward(&mut self){
        let head = self.body.back().unwrap();
        //there will be provlems since .back() returns a type Option object
        let new_point = head.go(self.direction); //see move function for point in game.rs
        self.body.push_back(new_point);
        self.body.pop_front();
        // we need testing for this
    }
    
    /* It adds a point on the head of the snake without getting rid of another point
    to be called whenever a snake is about to encounter an apple
    Remember that the snake must be pointing towards the apple*/
    pub fn grow(&mut self){ 
        // we will use point.move for this as well
        let head = self.body.back().unwrap();
        let new_point = head.go(self.direction);
        self.body.push_back(new_point);
    }
    /* It changes the direction of the snake without moving any of its points
    This function must be called when turning the snake and it happens within two forwards */
    pub fn turn(&mut self, dir: Dir){
        //self.direction = Dir::dir;
        match dir {
            Dir::RIGHT => self.direction = Dir::RIGHT,
            Dir::LEFT => self.direction = Dir::LEFT,
            Dir::UP => self.direction = Dir::UP,
            Dir::DOWN => self.direction = Dir::DOWN,
        }
    }
}