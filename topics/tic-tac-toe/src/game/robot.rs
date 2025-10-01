use crate::game::board::Board;
use rand::Rng;

pub struct Robot {
    symbol: char,
}

impl Robot {
    pub fn new(symbol: char) -> Self {
        Robot { symbol }
    }

    pub fn make_move(&self, board: &mut Board) -> Option<usize> {
        let available_positions = self.get_available_positions(board);
        
        if available_positions.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..available_positions.len());
        let chosen_position = available_positions[random_index];

        if board.place_symbol_by_position(chosen_position, self.symbol) {
            Some(chosen_position)
        } else {
            None
        }
    }

    fn get_available_positions(&self, board: &Board) -> Vec<usize> {
        let mut available = Vec::new();
        
        for position in 1..=9 {
            let (row, col) = self.position_to_coords(position);
            if board.is_position_empty(row, col) {
                available.push(position);
            }
        }
        
        available
    }

    fn position_to_coords(&self, position: usize) -> (usize, usize) {
        let index = position - 1; // Convert to 0-based index
        let row = index / 3;
        let col = index % 3;
        (row, col)
    }
}