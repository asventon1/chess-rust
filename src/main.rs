mod game;
use crate::game::Board;

fn main() {
    println!("Hello, world!");

    let board = Board::new_from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")).expect("");

}
