//use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
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
    /// と
    PPawn,
    /// 成香
    PLance,
    /// 成桂
    PKnight,
    /// 成銀
    PSilver,
    /// 馬
    Horse,
    /// 龍
    Dragon,
}

impl Piece {
    pub fn is_promotable(self) -> bool {
        match self {
            Piece::Pawn => true,
            Piece::Lance => true,
            Piece::Knight => true,
            Piece::Silver => true,
            Piece::Bishop => true,
            Piece::Rook => true,
            _ => false,
        }
    }

    pub fn promote(self) -> Piece {
        match self {
            Piece::Pawn => Piece::PPawn,
            Piece::Lance => Piece::PLance,
            Piece::Knight => Piece::PKnight,
            Piece::Silver => Piece::PSilver,
            Piece::Bishop => Piece::Horse,
            Piece::Rook => Piece::Dragon,
            _ => self,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(piece_to_csa(Piece::Pawn), "FU");
        assert_eq!(Piece::Bishop.promote(), Piece::Horse);
    }
}
