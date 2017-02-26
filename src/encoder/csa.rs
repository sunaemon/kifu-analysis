use super::super::types::*;

pub fn piece(p: Piece) -> String {
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
