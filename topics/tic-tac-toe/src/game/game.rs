use crate::game::board::Board;

pub struct Game {
    board: Board
}

impl Game {
    pub fn new() -> Self {
        println!("Starting a new game of Tic Tac Toe!");
        Game {
            board: Board::new()
        }
    }

    pub fn play(&mut self) {
        self.display_board();
        
    }

    pub fn display_board(&self) {
        self.board.display();
    }
}