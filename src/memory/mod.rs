pub mod colour;
pub mod position;

use colour::{Colour, BLACK};
use position::Position;

#[derive(Debug)]
pub struct Memory {
    board: Vec<Colour>,
    width: usize,
    height: usize,
}

impl Memory {
    pub fn new(width: usize, height: usize) -> Self {
        Memory {
            board: vec![BLACK; width * height],
            height,
            width,
        }
    }

    ///
    /// Sets a pixel on the grid at the position supplied
    /// Returns the original colour at that position
    ///
    pub fn set(&mut self, pos: Position, col: Colour) -> Colour {
        let position = pos.y * self.width + pos.x;
        let original = self.board[position];
        self.board[position] = col;
        original
    }

    ///
    /// Gets a pixel from the grid at the specified position
    ///
    pub fn get(&self, pos: Position) -> Colour {
        let position = pos.y * self.width + pos.x;
        self.board[position]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_inits_board_of_right_size() {
        let size = 10;
        let memory = Memory::new(size, size);
        let expected = vec![BLACK; size * size];
        assert_eq!(expected.len(), memory.board.len());
    }

    #[test]
    fn it_inits_empty_board() {
        let size = 10;
        let memory = Memory::new(size, size);
        let expected = vec![BLACK; size * size];

        let it = expected.iter().zip(memory.board.iter());
        for (_i, (expect, received)) in it.enumerate() {
            assert_eq!(expect, received);
        }
    }

    #[test]
    fn it_sets_a_pixel() {
        let size = 3;
        let mut memory = Memory::new(size, size);
        let mut expected = vec![BLACK; size * size];
        expected[4] = Colour::new(0, 255, 0);

        let original = memory.set(Position { x: 1, y: 1 }, Colour::new(0, 255, 0));
        assert_eq!(original, BLACK);

        let it = expected.iter().zip(memory.board.iter());
        for (_i, (expect, received)) in it.enumerate() {
            assert_eq!(expect, received);
        }
    }

    #[test]
    fn it_reads_a_pixel() {
        let size = 3;
        let mut memory = Memory::new(size, size);
        memory.board[4] = Colour::new(255, 0, 0);

        let col = memory.get(Position { x: 1, y: 1 });
        assert_eq!(col, Colour::new(255, 0, 0));
    }
}
