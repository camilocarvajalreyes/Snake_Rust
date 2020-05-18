// Structure du serpent
use crate::game::*; // importing structures and objects from game.rs (?)
use std::collections::VecDeque;
pub struct Snake { //construction of the strut snake
    body: VecDeque<Point>, //body of the snake as a list of Points
    direction: i32, //direction 0 if up, 1 left, 2, down, 3 right (?)
}
impl Snake {
    pub fn newSnake(initialPoint: Point) -> Self { 
        let mut vec = VecDeque::new();
        vec.push_back(initialPoint);
        Snake {vec, 0} //function that returns a single point snake going up as default
    }
    pub fn forward(&mut self){ //adds a point in the actual direction to the back (head)
        // then deletes the element in the front (tail)
        let head = self.body.last();
        //there will be provlems since .last() returns a type Option object
        let newPoint = head.move(self.direction); //see move function for point in game.rs
        self.body.push_back(newPoint);
        self.body.pop_front();
        // we need testing for this
    }
    pub fn grow(){} // we will use point.move for this as well
}