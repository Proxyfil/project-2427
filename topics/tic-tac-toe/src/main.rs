mod game;

use crate::game::game::Game;

fn main() {
    println!("Starting Tic Tac Toe game..." );
    let mut game = Game::new();

    game.play();
}
