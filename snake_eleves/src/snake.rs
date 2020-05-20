// Structure du serpent
use crate::game::*; 
//pub mod game;// importing structures and objects from game.rs (?)
use std::collections::VecDeque;

pub struct Snake { //construction of the strut snake
    pub body: VecDeque<Point>, //body of the snake as a list of Points
    pub direction: Dir, //direction 0 if up, 1 left, 2, down, 3 right (?)
}
impl Snake {
    pub fn new(initialPoint: Point) -> Self { 
        let mut vec = VecDeque::new();
        vec.push_back(initialPoint);
        Snake {body: vec, direction: Dir::UP} //function that returns a single point snake going up as default
    }

    /* adds a point in the actual direction to the back (head)
       then deletes the element in the front (tail) 
    */
    /*
    pub fn forward(&mut self){
        let head = self.body.back().unwrap();
        //there will be provlems since .back() returns a type Option object
        let newPoint = head.go(self.direction); //see move function for point in game.rs
        self.body.push_back(newPoint);
        self.body.pop_front();
        // we need testing for this
    }
    
    /* It adds a point on the head of the snake without getting rid of another point
    to be called whenever a snake is about to encounter an apple*/
    pub fn grow(&mut self){ 
        // we will use point.move for this as well
        let head = self.body.back().unwrap();
        let newPoint = head.go(self.direction);
        self.body.push_back(newPoint);
    }
    */
}