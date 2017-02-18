use super::piece::Piece;
use super::piece::Color;

#[derive(PartialEq, Debug)]
pub struct Position {
    board: [[Option<(Color, Piece)>; 9]; 9],
    captured: [Vec<Piece>; 2],
    c: Color,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        assert!(x < 9 && y < 9);
        Point { x: x, y: y }
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
pub struct Move {
    pub c: Color,
    pub from: Point,
    pub to: Point,
    pub p: Piece,
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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Step {
    Move(Move),
    IssueOfGame(IssueOfGame),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
