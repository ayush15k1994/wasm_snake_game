use std::collections::VecDeque;

use crate::random::random_range;

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    UP, RIGHT, DOWN, LEFT
}

 #[derive(Debug)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>,  // head is the first item and tail is the last item in the vector
    pub direction: Direction,
    next_direction: Direction, 
    pub food: Position,
    pub finished: bool,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self{
            width, height, 
            snake: [((width-3).max(0), height/2)].into_iter().collect(),
            direction: Direction::LEFT,
            next_direction: Direction::LEFT,
            food: (2.min(width-1), height/2),
            finished: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.finished {
            return;
        }

        match (&self.direction, direction) {
            (Direction::UP, Direction::UP) |
            (Direction::UP, Direction::DOWN) |
            (Direction::RIGHT, Direction::RIGHT) |
            (Direction::RIGHT, Direction::LEFT) |
            (Direction::DOWN, Direction::UP) |
            (Direction::DOWN, Direction::DOWN) |
            (Direction::LEFT, Direction::RIGHT) |
            (Direction::LEFT, Direction::LEFT) => {}

            (_, direction) => {
                self.next_direction = direction;
            }
        }
    }

    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        if self.finished && self.snake.len() == 0 {
            return;
        }

        self.direction = self.next_direction;

        let (x, y) = self.snake[0];

        let new_head = match &self.direction {
            Direction::UP => (x, y-1),
            Direction::RIGHT => (x+1, y),
            Direction::DOWN => (x, y+1),
            Direction::LEFT => (x-1, y),
        };
        
        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.finished = true
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                let free_positions = (0..self.height)
                        .flat_map(|y| (0..self.width)
                        .map(move |x| (x, y)))
                        .filter(|pos| !self.snake.contains(pos))
                        .collect::<Vec<_>>();

                if free_positions.is_empty() {
                    self.finished = true;
                    return;
                }

                self.food = free_positions[random_range(0, free_positions.len())];
            }
            self.snake.push_front(new_head);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SnakeGame;

    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10, 10))
    }
}