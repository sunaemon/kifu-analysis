use piece::Piece;
use piece::Color;
use std::fmt;

#[derive(PartialEq)]
pub struct Position {
    board: [[Option<(Color, Piece)>; 9]; 9],
    captured: [Vec<Piece>; 2],
    /// teban
    c: Color,
}

fn sfen_to_piece(s: &str) -> Option<(Color, Piece)> {
    Some((Color::White, Piece::Pawn))
}

pub fn sfen_to_position(s: &str) -> Result<Position, String> {
    let mut board: [[Option<(Color, Piece)>; 9]; 9] = [[None; 9]; 9];
    let mut i = 0;
    let mut j = 0;

    for c in s.chars() {
        if c.is_numeric() {
            i += c.to_digit(10).unwrap();
            if i > 9 {
                return Err("board overflow".to_string());
            }
        }

        println!("{}", c)
    }

    Ok(Position {
        board: board,
        captured: [vec![], vec![]],
        c: Color::White,
    })
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use piece::Piece;
    use piece::Color;
    #[test]
    fn it_works() {
        assert_eq!(sfen_to_position("p8/9/9/9/8P/9/9/9/9"),
                   Ok(Position {
                       board: [[Some((Color::Black, Piece::Pawn)),
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None],
                               [None, None, None, None, None, None, None, None, None],
                               [None, None, None, None, None, None, None, None, None],
                               [None, None, None, None, None, None, None, None, None],
                               [None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                None,
                                Some((Color::White, Piece::Pawn))],
                               [None, None, None, None, None, None, None, None, None],
                               [None, None, None, None, None, None, None, None, None],
                               [None, None, None, None, None, None, None, None, None],
                               [None, None, None, None, None, None, None, None, None]],
                       captured: [vec![], vec![]],
                       c: Color::White,
                   }));
    }
}
