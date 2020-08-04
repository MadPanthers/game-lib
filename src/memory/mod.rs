mod colour;

#[derive(Debug)]
pub struct Memory {
    board: Vec<colour::Colour>,
}

impl Memory {
    pub fn new(width: usize, height: usize) -> Self {
        Memory {
            board: vec![colour::BLACK; width * height],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_inits_board_of_right_size() {
        let size = 10;
        let memory = Memory::new(size, size);
        let expected = vec![colour::BLACK; size * size];
        assert_eq!(expected.len(), memory.board.len());
    }

    #[test]
    fn it_inits_empty_board() {
        let size = 10;
        let memory = Memory::new(size, size);
        let expected = vec![colour::BLACK; size * size];

        let it = expected.iter().zip(memory.board.iter());
        for (_i, (expect, received)) in it.enumerate() {
            assert_eq!(expect, received);
        }
    }
}
