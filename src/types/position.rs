use super::piece::Piece;
use std::collections::BTreeMap;
use std::ops::Index;
use std::ops::IndexMut;

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

#[derive(PartialEq, Debug)]
pub struct Board {
    data: [[Option<(Color, Piece)>; 9]; 9],
}

impl Board {
    pub fn new(d: [[Option<(Color, Piece)>; 9]; 9]) -> Board {
        Board { data: d }
    }

    pub fn hirate() -> Board {
        Board::new([[Some((Color::White, Piece::Lance)),
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
                     Some((Color::Black, Piece::Lance))]])
    }
}

impl Index<Point> for Board {
    type Output = Option<(Color, Piece)>;
    fn index(&self, index: Point) -> &Option<(Color, Piece)> {
        &self.data[index.x as usize][index.y as usize]
    }
}

impl IndexMut<Point> for Board {
    fn index_mut(&mut self, index: Point) -> &mut Option<(Color, Piece)> {
        &mut self.data[index.x as usize][index.y as usize]
    }
}

#[derive(PartialEq, Debug)]
pub struct Captured {
    data: BTreeMap<(Color, Piece), u8>,
}

impl Captured {
    pub fn new() -> Captured {
        Captured { data: BTreeMap::new() }
    }
    pub fn how_many(&self, c: Color, p: Piece) -> u8 {
        if let Some(n) = self.data.get(&(c, p)) {
            *n
        } else {
            0
        }
    }
    pub fn has(&self, c: Color, p: Piece) -> bool {
        self.how_many(c, p) > 0
    }
    pub fn consume(&mut self, c: Color, p: Piece) -> Option<()> {
        if let Some(n) = self.data.get_mut(&(c, p)) {
            if *n > 1 {
                *n -= 1;
                return Some(());
            } else {
                None
            }

        } else {
            None
        }
    }
    pub fn add(&mut self, c: Color, p: Piece) {
        *self.data.entry((c, p)).or_insert(0) += 1
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl From<BTreeMap<(Color, Piece), u8>> for Captured {
    fn from(data: BTreeMap<(Color, Piece), u8>) -> Self {
        Captured { data: data }
    }
}

impl<'a> IntoIterator for &'a Captured {
    type Item = (&'a (Color, Piece), &'a u8);
    type IntoIter = ::std::collections::btree_map::Iter<'a, (Color, Piece), u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

#[derive(PartialEq, Debug)]
pub struct Position {
    board: Board,
    captured: Captured,
    c: Color,
}

impl Position {
    pub fn new(board: Board, captured: Captured, c: Color) -> Position {
        Position {
            board: board,
            captured: captured,
            c: c,
        }
    }
    pub fn hirate() -> Position {
        Position {
            board: Board::hirate(),
            captured: Captured::new(),
            c: Color::Black,
        }
    }
    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn captured(&self) -> &Captured {
        &self.captured
    }
    pub fn color(&self) -> Color {
        self.c
    }

    /// positionが壊れない程度に正しいmoveか？
    pub fn move_valid(&self, m: &Move) -> bool {
        if m.color() != self.color() {
            return false;
        }

        if let Some(p) = m.from() {
            if self.board()[p] != Some((m.color(), m.piece())) {
                return false;
            }
        } else {
            if !self.captured.has(m.color(), m.piece()) {
                return false;
            }
        }

        true
    }
    pub fn make_move(&mut self, m: &Move) -> Option<()> {
        if !self.move_valid(m) {
            return None;
        }

        if let Some(from) = m.from() {
            self.board[from] = None
        } else {
            self.captured.consume(m.color(), m.piece()).unwrap()
        }

        if let Some((c, p)) = self.board()[m.to()] {
            self.captured.add(m.color(), p)
        }

        self.board[m.to()] = Some((m.color(), m.piece()));
        None
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
    use super::*;
    use super::super::piece::Piece;
    #[test]
    fn it_works() {
        let mut p = Position::hirate();
        p.make_move(&Move::new(Color::Black,
                               Some(Point::new(7, 7)),
                               Point::new(7, 6),
                               Piece::Pawn,
                               false)
            .unwrap());

        assert_eq!(p, Position::hirate());
    }
}
