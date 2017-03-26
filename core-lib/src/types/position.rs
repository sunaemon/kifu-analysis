use super::piece::Piece;
use std::collections::BTreeMap;
use std::ops::Index;
use std::ops::IndexMut;
use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub enum Color {
    ///先手
    Black,
    ///後手
    White,
}

impl Color {
    pub fn another(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct Point {
    /// 段
    pub x: u8,
    /// 筋
    pub y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        assert!(x < 9 && y < 9);
        Point { x: x, y: y }
    }
    pub fn one_start(x: u8, y: u8) -> Point {
        Point::new(x - 1, y - 1)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point{{ {}{} }}", self.x + 1, self.y + 1)
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
    pub fn new(c: Color,
               from: Option<Point>,
               to: Point,
               p: Piece,
               promote: bool)
               -> Result<Move, String> {
        if promote && !p.is_promoted() {
            return Err(format!("{:?} is not promoted", p));
        }

        Ok(Move {
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
        !self.from.is_some()
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
    pub fn piece_before_move(&self) -> Piece {
        if self.is_promote() {
            self.p.demote().unwrap()
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
    // 相手の接続切れで勝ち
    //Disconnect,
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

#[derive(PartialEq, Debug, Clone)]
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
                     Some((Color::White, Piece::Bishop)),
                     None,
                     None,
                     None,
                     None,
                     None,
                     Some((Color::White, Piece::Rook)),
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
                     Some((Color::Black, Piece::Rook)),
                     None,
                     None,
                     None,
                     None,
                     None,
                     Some((Color::Black, Piece::Bishop)),
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
        &self.data[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Point> for Board {
    fn index_mut(&mut self, index: Point) -> &mut Option<(Color, Piece)> {
        &mut self.data[index.y as usize][index.x as usize]
    }
}

#[derive(PartialEq, Debug, Clone)]
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
            if *n > 0 {
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

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    board: Board,
    captured: Captured,
    color: Color,
}

impl Position {
    #[allow(dead_code)]
    pub fn new(board: Board, captured: Captured, c: Color) -> Position {
        Position {
            board: board,
            captured: captured,
            color: c,
        }
    }
    pub fn hirate() -> Position {
        Position {
            board: Board::hirate(),
            captured: Captured::new(),
            color: Color::Black,
        }
    }
    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn captured(&self) -> &Captured {
        &self.captured
    }
    pub fn color(&self) -> Color {
        self.color
    }

    /// positionが壊れない程度に正しいmoveか？
    pub fn move_valid(&self, m: &Move) -> Result<(), String> {
        if m.color() != self.color() {
            return Err("color check failed".to_string());
        }


        if let Some(p) = m.from() {
            if self.board()[p] != Some((m.color(), m.piece_before_move())) {
                println!("Board: {:?}", self.board());
                return Err(format!("from check failed {:?}(at {:?} is not {:?}",
                                   self.board()[p],
                                   p,
                                   (m.color(), m.piece_before_move())));
            }
        } else {
            if !self.captured.has(m.color(), m.piece()) {
                return Err("drop check failed".to_string());
            }
        }

        Ok(())
    }
    pub fn make_move(&mut self, m: &Move) -> Result<(), String> {
        try!(self.move_valid(m));

        if let Some(from) = m.from() {
            self.board[from] = None
        } else {
            self.captured.consume(m.color(), m.piece()).unwrap()
        }

        if let Some((_, p)) = self.board()[m.to()] {
            self.captured.add(m.color(), p.demote().unwrap_or(p))
        }

        self.board[m.to()] = Some((m.color(), m.piece()));

        self.color = self.color.another();
        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Game {
    pub position: Position,
    pub moves: Vec<Move>,
    pub issue: Option<IssueOfGame>,
}

impl Game {
    pub fn hirate() -> Game {
        Game {
            position: Position::hirate(),
            moves: Vec::new(),
            issue: None,
        }
    }
}

#[cfg(test)]
mod tests {}
