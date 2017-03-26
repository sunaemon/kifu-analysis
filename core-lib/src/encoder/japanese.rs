use types::*;

fn zenkaku(i: u8) -> String {
    match i {
            1 => "１",
            2 => "２",
            3 => "３",
            4 => "４",
            5 => "５",
            6 => "６",
            7 => "７",
            8 => "８",
            9 => "９",
            _ => panic!(),
        }
        .to_string()
}

fn kanji(i: u8) -> String {
    match i {
            1 => "一",
            2 => "二",
            3 => "三",
            4 => "四",
            5 => "五",
            6 => "六",
            7 => "七",
            8 => "八",
            9 => "九",
            _ => panic!(),
        }
        .to_string()
}

fn color(c: Color) -> String {
    match c {
            Color::Black => "▲",
            Color::White => "△",
        }
        .to_string()
}

fn piece(p: Piece) -> String {
    match p {
            Piece::Pawn => "歩",
            Piece::Lance => "香",
            Piece::Knight => "桂",
            Piece::Silver => "銀",
            Piece::Gold => "金",
            Piece::Bishop => "角",
            Piece::Rook => "飛",
            Piece::King => "玉",
            Piece::PPawn => "と",
            Piece::PLance => "成香",
            Piece::PKnight => "成桂",
            Piece::PSilver => "成銀",
            Piece::Horse => "馬",
            Piece::Dragon => "龍",
        }
        .to_string()
}

pub fn enc_move(m: &Move) -> String {
    format!("{}{}{}{}{}",
            color(m.color()),
            zenkaku(m.to().x + 1),
            kanji(m.to().y + 1),
            piece(m.piece()),
            if m.is_drop() { "打" } else { "" })
}
