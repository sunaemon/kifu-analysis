use super::piece::Piece;
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum Color {
    ///先手
    Black = 0,
    ///後手
    White = 1,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        assert!(x < 9 && y < 9);
        Point { x: x, y: y }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Move {
    c: Color,
    from: Option<Point>,
    to: Point,
    p: Piece,
    promote: bool,
}

impl Move {
    pub fn new(c: Color, from: Option<Point>, to: Point, p: Piece, promote: bool) -> Option<Move> {
        if promote && !p.is_promotable() {
            return None;
        }

        Some(Move {
            c: c,
            from: from,
            to: to,
            p: p,
            promote: promote,
        })
    }
    pub fn color(&self) -> Color {
        self.c
    }
    pub fn is_drop(&self) -> bool {
        self.from.is_some()
    }
    pub fn is_promote(&self) -> bool {
        self.promote
    }
    pub fn from(&self) -> Option<Point> {
        self.from
    }
    pub fn to(&self) -> Point {
        self.to
    }
    pub fn piece(&self) -> Piece {
        self.p
    }
    pub fn piece_after_move(&self) -> Piece {
        if self.promote {
            self.p.promote().unwrap()
        } else {
            self.p
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Win {
    /// 相手の投了で勝ち
    Toryo,
    /// 相手の玉を詰ませて勝ち
    Checkmate,
    /// 相手の時間切れで勝ち
    Timeout,
    /// 相手の接続切れで勝ち
    Disconnect,
    /// 入玉で勝ち
    EnteringKing,
    /// 相手の連続王手の千日手による反則負けで勝ち
    OuteSennnichi,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Draw {
    /// 千日手で引き分け
    Sennnichi,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum IssueOfGame {
    Win(Color, Win),
    Draw(Draw),
}

pub type Board = [[Option<(Color, Piece)>; 9]; 9];

pub fn hirate_board() -> Board {
    [[Some((Color::White, Piece::Lance)),
      Some((Color::White, Piece::Knight)),
      Some((Color::White, Piece::Silver)),
      Some((Color::White, Piece::Gold)),
      Some((Color::White, Piece::King)),
      Some((Color::White, Piece::Gold)),
      Some((Color::White, Piece::Silver)),
      Some((Color::White, Piece::Knight)),
      Some((Color::White, Piece::Lance))],
     [None,
      Some((Color::White, Piece::Rook)),
      None,
      None,
      None,
      None,
      None,
      Some((Color::White, Piece::Bishop)),
      None],
     [Some((Color::White, Piece::Pawn)),
      Some((Color::White, Piece::Pawn)),
      Some((Color::White, Piece::Pawn)),
      Some((Color::White, Piece::Pawn)),
      Some((Color::White, Piece::Pawn)),
      Some((Color::White, Piece::Pawn)),
      Some((Color::White, Piece::Pawn)),
      Some((Color::White, Piece::Pawn)),
      Some((Color::White, Piece::Pawn))],
     [None, None, None, None, None, None, None, None, None],
     [None, None, None, None, None, None, None, None, None],
     [None, None, None, None, None, None, None, None, None],
     [Some((Color::Black, Piece::Pawn)),
      Some((Color::Black, Piece::Pawn)),
      Some((Color::Black, Piece::Pawn)),
      Some((Color::Black, Piece::Pawn)),
      Some((Color::Black, Piece::Pawn)),
      Some((Color::Black, Piece::Pawn)),
      Some((Color::Black, Piece::Pawn)),
      Some((Color::Black, Piece::Pawn)),
      Some((Color::Black, Piece::Pawn))],
     [None,
      Some((Color::Black, Piece::Bishop)),
      None,
      None,
      None,
      None,
      None,
      Some((Color::Black, Piece::Rook)),
      None],
     [Some((Color::Black, Piece::Lance)),
      Some((Color::Black, Piece::Knight)),
      Some((Color::Black, Piece::Silver)),
      Some((Color::Black, Piece::Gold)),
      Some((Color::Black, Piece::King)),
      Some((Color::Black, Piece::Gold)),
      Some((Color::Black, Piece::Silver)),
      Some((Color::Black, Piece::Knight)),
      Some((Color::Black, Piece::Lance))]]
}

#[derive(PartialEq, Debug)]
pub struct Position {
    board: Board,
    captured: BTreeMap<(Color, Piece), u8>,
    c: Color,
}

impl Position {
    pub fn new(board: Board, captured: BTreeMap<(Color, Piece), u8>, c: Color) -> Position {
        Position {
            board: board,
            captured: captured,
            c: c,
        }
    }
    pub fn hirate() -> Position {
        Position {
            board: hirate_board(),
            captured: BTreeMap::new(),
            c: Color::Black,
        }
    }
    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn captured(&self) -> &BTreeMap<(Color, Piece), u8> {
        &self.captured
    }
    pub fn color(&self) -> Color {
        self.c
    }
    pub fn make_move(&mut self, m: &Move) -> Option<()> {
        match m.from {
            Some(p) => Some(()),
            None => None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Game {
    pub pos: Position,
    pub moves: Vec<Move>,
    pub issue: Option<IssueOfGame>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
