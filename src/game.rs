
#[derive(PartialEq, Debug, Copy, Clone)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum PieceColor {
    Black,
    White,
}

#[derive(Debug, PartialEq)]
struct Piece {
    x: u32,
    y: u32,
    ptype: PieceType,
    color: PieceColor,
}

#[derive(Debug, PartialEq)]
pub struct Board {
    pieces: Vec<Piece>,

    // Who's move is it
    current_move: PieceColor,

    // These are true if the piece may be able to castle in the future,
    // and false if they can no longer castle.
    can_white_king_castle: bool,
    can_white_queen_castle: bool,
    can_black_king_castle: bool,
    can_black_queen_castle: bool,

    // If a pawn has just moved two squares the square behind it's current position
    // goes in this variable. This is used for en passant.
    en_passant_square: Option<(u8, u8)>,

    // Number of halfmoves since the last capture (a halfmove is everytime one person plays)
    halfmove_clock: u8,

    // Number of fullmoves since the start of the game.
    // Starts at one and is incremented after black plays.
    fullmove_num: u32,
}

fn char_to_piece_type(c: char) -> Result<PieceType, String> {
    match c {
        'p' => Ok(PieceType::Pawn),
        'r' => Ok(PieceType::Rook),
        'n' => Ok(PieceType::Knight),
        'b' => Ok(PieceType::Bishop),
        'q' => Ok(PieceType::Queen),
        'k' => Ok(PieceType::King),
        _ => Err(String::from("Invalid piece char")),
    }
}

fn piece_type_to_char(c: PieceType) -> char {
    match c {
        PieceType::Pawn => 'p',
        PieceType::Rook => 'r',
        PieceType::Knight => 'n',
        PieceType::Bishop => 'b',
        PieceType::Queen => 'q',
        PieceType::King => 'k',
    }
}

fn square_from_string(s: String) -> Result<(u8, u8), String> {
    if s.len() != 2 { return Err(String::from("Invalid string to turn into square.")); }
    Ok((
        (s.as_bytes()[0] as u32 - 'a' as u32) as u8,
        match (s.as_bytes()[1] as char).to_digit(10) {
            None => return Err(String::from("Invalid string to turn into square.")),
            Some(n) => (n - 1) as u8,
        }
    ))
}

impl Piece {

}

impl Board {
    // Makes a new board from a FEN notation.
    pub fn new_from_fen(fen: String) -> Result<Board, String> {
        let mut pieces = Vec::<Piece>::new();
        let mut x = 0;
        let mut y = 0;
        let fen_vec: Vec<&str> = fen.split(" ").collect();
        for v in fen_vec[0].chars() {
            if v.is_digit(10) {
                x += v.to_digit(10).unwrap();
                continue;
            }
            if v == '/' {
                y += 1;
                x = 0;
                continue;
            }
            let piece = match char_to_piece_type(v.to_ascii_lowercase()) {
                Err(_) => return Err(String::from("Invalid fen")),
                Ok(p) =>
                    Piece {
                        x,
                        y,
                        ptype: p,
                        color: if v.is_ascii_lowercase() { PieceColor::Black } else { PieceColor::White },
                    }
            };
            pieces.push(piece);
            x += 1;
        }
        Ok(Board {
            pieces,
            current_move:
            if fen_vec[1] == "w" { PieceColor::White } else if fen_vec[1] == "w" { PieceColor::Black } else { return Err(String::from("Invalid fen")); },
            can_white_king_castle: fen_vec[2].contains("K"),
            can_white_queen_castle: fen_vec[2].contains("Q"),
            can_black_king_castle: fen_vec[2].contains("k"),
            can_black_queen_castle: fen_vec[2].contains("q"),
            en_passant_square: if fen_vec[3] == "-" { None } else {
                match square_from_string(String::from(fen_vec[3])) {
                    Err(_) => return Err(String::from("Invalid fen")),
                    Ok(s) => Some(s),
                }
            },
            halfmove_clock: fen_vec[4].parse().unwrap(),
            fullmove_num: fen_vec[5].parse().unwrap(),
        })
    }

    pub fn render(&self) {
        for y in 0..8 {
            for x in 0..8 {
                let mut piece_on_square = false;
                for v in &self.pieces {
                    if v.x == x && v.y == y {
                        let c = piece_type_to_char(v.ptype);
                        print!("{} ", if v.color == PieceColor::White { c.to_uppercase().to_string() } else { c.to_string() });
                        piece_on_square = true;
                        break;
                    }
                }
                if !piece_on_square {
                    print!("* ");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_fen() {
        let board = Board::new_from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")).expect("");
        assert_eq!(Board {
            pieces: vec![Piece { x: 0, y: 0, ptype: PieceType::Rook, color: PieceColor::Black },
                         Piece { x: 1, y: 0, ptype: PieceType::Knight, color: PieceColor::Black },
                         Piece { x: 2, y: 0, ptype: PieceType::Bishop, color: PieceColor::Black },
                         Piece { x: 3, y: 0, ptype: PieceType::Queen, color: PieceColor::Black },
                         Piece { x: 4, y: 0, ptype: PieceType::King, color: PieceColor::Black },
                         Piece { x: 5, y: 0, ptype: PieceType::Bishop, color: PieceColor::Black },
                         Piece { x: 6, y: 0, ptype: PieceType::Knight, color: PieceColor::Black },
                         Piece { x: 7, y: 0, ptype: PieceType::Rook, color: PieceColor::Black },
                         Piece { x: 0, y: 1, ptype: PieceType::Pawn, color: PieceColor::Black },
                         Piece { x: 1, y: 1, ptype: PieceType::Pawn, color: PieceColor::Black },
                         Piece { x: 2, y: 1, ptype: PieceType::Pawn, color: PieceColor::Black },
                         Piece { x: 3, y: 1, ptype: PieceType::Pawn, color: PieceColor::Black },
                         Piece { x: 4, y: 1, ptype: PieceType::Pawn, color: PieceColor::Black },
                         Piece { x: 5, y: 1, ptype: PieceType::Pawn, color: PieceColor::Black },
                         Piece { x: 6, y: 1, ptype: PieceType::Pawn, color: PieceColor::Black },
                         Piece { x: 7, y: 1, ptype: PieceType::Pawn, color: PieceColor::Black },
                         Piece { x: 0, y: 6, ptype: PieceType::Pawn, color: PieceColor::White },
                         Piece { x: 1, y: 6, ptype: PieceType::Pawn, color: PieceColor::White },
                         Piece { x: 2, y: 6, ptype: PieceType::Pawn, color: PieceColor::White },
                         Piece { x: 3, y: 6, ptype: PieceType::Pawn, color: PieceColor::White },
                         Piece { x: 4, y: 6, ptype: PieceType::Pawn, color: PieceColor::White },
                         Piece { x: 5, y: 6, ptype: PieceType::Pawn, color: PieceColor::White },
                         Piece { x: 6, y: 6, ptype: PieceType::Pawn, color: PieceColor::White },
                         Piece { x: 7, y: 6, ptype: PieceType::Pawn, color: PieceColor::White },
                         Piece { x: 0, y: 7, ptype: PieceType::Rook, color: PieceColor::White },
                         Piece { x: 1, y: 7, ptype: PieceType::Knight, color: PieceColor::White },
                         Piece { x: 2, y: 7, ptype: PieceType::Bishop, color: PieceColor::White },
                         Piece { x: 3, y: 7, ptype: PieceType::Queen, color: PieceColor::White },
                         Piece { x: 4, y: 7, ptype: PieceType::King, color: PieceColor::White },
                         Piece { x: 5, y: 7, ptype: PieceType::Bishop, color: PieceColor::White },
                         Piece { x: 6, y: 7, ptype: PieceType::Knight, color: PieceColor::White },
                         Piece { x: 7, y: 7, ptype: PieceType::Rook, color: PieceColor::White }],
            current_move: PieceColor::White,
            can_white_king_castle: true,
            can_white_queen_castle: true,
            can_black_king_castle: true,
            can_black_queen_castle: true,
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_num: 1,
        }, board);
    }

    #[test]
    fn test_char_to_piece_type() {
        assert_eq!(char_to_piece_type('p'), Ok(PieceType::Pawn));
        assert_eq!(char_to_piece_type('r'), Ok(PieceType::Rook));
        assert_eq!(char_to_piece_type('n'), Ok(PieceType::Knight));
        assert_eq!(char_to_piece_type('b'), Ok(PieceType::Bishop));
        assert_eq!(char_to_piece_type('q'), Ok(PieceType::Queen));
        assert_eq!(char_to_piece_type('k'), Ok(PieceType::King));
        assert_eq!(char_to_piece_type('z'), Err(String::from("Invalid piece char")));
        assert_eq!(char_to_piece_type('a'), Err(String::from("Invalid piece char")));
    }
}