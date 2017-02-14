#[warn(dead_code)]

use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum Piece {
    /// 歩
    Pawn,
    /// 香
    Lance,
    /// 桂
    Knight,
    /// 銀
    Silver,
    /// 金
    Gold,
    /// 角
    Bishop,
    /// 飛
    Rook,
    /// 玉
    King,
    ///と
    PPawn,
    ///成香
    PLance,
    ///成桂
    PKnight,
    ///成銀
    PSilver,
    ///馬
    Horse,
    ///龍
    Dragon,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

pub fn piece_to_csa(p: Piece) -> String {
    match p {
            Piece::Pawn => "FU",
            Piece::Lance => "KY",
            Piece::Knight => "KE",
            Piece::Silver => "GI",
            Piece::Gold => "KI",
            Piece::Bishop => "KA",
            Piece::Rook => "HI",
            Piece::King => "OU",
            Piece::PPawn => "TO",
            Piece::PLance => "NY",
            Piece::PKnight => "NK",
            Piece::PSilver => "NG",
            Piece::Horse => "UM",
            Piece::Dragon => "RY",
        }
        .to_string()
}

pub fn usi_to_piece(p: Piece) -> String {
    match p {
            Piece::Pawn => "p",
            Piece::Lance => "l",
            Piece::Knight => "n",
            Piece::Silver => "s",
            Piece::Gold => "g",
            Piece::Bishop => "b",
            Piece::Rook => "r",
            Piece::King => "k",
            Piece::PPawn => "+p",
            Piece::PLance => "+l",
            Piece::PKnight => "+n",
            Piece::PSilver => "+s",
            Piece::Horse => "+r",
            Piece::Dragon => "+b",
        }
        .to_string()
}

pub fn piece_to_usi(p: Piece) -> String {
    match p {
            Piece::Pawn => "p",
            Piece::Lance => "l",
            Piece::Knight => "n",
            Piece::Silver => "s",
            Piece::Gold => "g",
            Piece::Bishop => "b",
            Piece::Rook => "r",
            Piece::King => "k",
            Piece::PPawn => "+p",
            Piece::PLance => "+l",
            Piece::PKnight => "+n",
            Piece::PSilver => "+s",
            Piece::Horse => "+r",
            Piece::Dragon => "+b",
        }
        .to_string()
}

pub fn piece_to_usi_with_color(p: Piece, c: Color) -> String {
    match c {
        Color::White => piece_to_usi(p),
        Color::Black => piece_to_usi(p).to_uppercase(),
    }
}

pub fn csa_to_piece(s: &str) -> Option<Piece> {
    match s {
        "FU" => Some(Piece::Pawn),
        "KY" => Some(Piece::Lance),
        "KE" => Some(Piece::Knight),
        "GI" => Some(Piece::Silver),
        "KI" => Some(Piece::Gold),
        "KA" => Some(Piece::Bishop),
        "HI" => Some(Piece::Rook),
        "OU" => Some(Piece::King),
        "TO" => Some(Piece::PPawn),
        "NY" => Some(Piece::PLance),
        "NK" => Some(Piece::PKnight),
        "NG" => Some(Piece::PSilver),
        "UM" => Some(Piece::Horse),
        "RY" => Some(Piece::Dragon),
        _ => None,
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", piece_to_csa(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(piece_to_csa(Piece::Pawn), "FU");
        assert_eq!(csa_to_piece("FU"), Some(Piece::Pawn))
    }
}
