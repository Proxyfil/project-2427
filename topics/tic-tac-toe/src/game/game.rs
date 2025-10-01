use crate::game::board::Board;
use std::io;

pub struct Game {
    board: Board,
    current_player: char,
}

impl Game {
    pub fn new() -> Self {
        println!("Starting a new game of Tic Tac Toe!");
        Game {
            board: Board::new(),
            current_player: 'X',
        }
    }

    pub fn play(&mut self) {
        loop {
            self.display_board();
            println!("Player {}'s turn", self.current_player);
            
            if self.get_player_move() {
                self.switch_player();
            }
        }
    }

    pub fn display_board(&self) {
        println!();
        self.board.display();
        println!();
    }

    fn get_player_move(&mut self) -> bool {
        println!("Enter position (1-9): ");
        let position = self.get_position_input();

        if self.board.place_symbol_by_position(position, self.current_player) {
            println!("Move placed successfully!");
            true
        } else {
            println!("Invalid move! Position is either occupied or invalid. Try again.");
            false
        }
    }

    fn get_position_input(&self) -> usize {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(num) if num >= 1 && num <= 9 => return num,
                _ => println!("Please enter a number between 1 and 9:"),
            }
        }
    }

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == 'X' { 'O' } else { 'X' };
    }
}