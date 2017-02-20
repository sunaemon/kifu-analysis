use types::*;
use std::fmt::Write;
use std::collections::BTreeMap;

pub fn piece(p: Piece) -> String {
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

pub fn piece_with_color(c: Color, p: Piece) -> String {
    match c {
        Color::White => piece(p),
        Color::Black => piece(p).to_uppercase(),
    }
}

pub fn board(b: &Board) -> String {
    let mut n = 0;
    let mut ret = String::new();
    for i in 0..9 {
        for j in 0..9 {
            match b[i][j] {
                Some((c, p)) => {
                    if n > 0 {
                        write!(ret, "{}", n).unwrap();
                        n = 0;
                    }
                    write!(ret, "{}", piece_with_color(c, p)).unwrap();
                }
                None => n += 1,
            }
        }
        if n > 0 {
            write!(ret, "{}", n).unwrap();
            n = 0;
        }
        if i != 8 {
            write!(ret, "/").unwrap();
        }
    }
    ret
}

pub fn captured(captured: &BTreeMap<(Color, Piece), u8>) -> String {
    let mut ret = String::new();
    if captured.is_empty() {
        write!(ret, "-").unwrap();
    }
    for (&t, &n) in captured {
        let (c, p) = t;
        if n == 1 {
            write!(ret, "{}", piece_with_color(c, p)).unwrap();
        } else if n > 1 {
            write!(ret, "{}{}", n, piece_with_color(c, p)).unwrap();
        }
    }
    ret
}

pub fn sfen(p: &Position) -> String {
    let mut ret = String::new();
    write!(&mut ret,
           "{} {} {} 1",
           board(&p.board),
           match p.c {
               Color::Black => "b",
               Color::White => "w",
           },
           captured(&p.captured))
        .unwrap();
    ret
}

fn dan(i: u8) -> String {
    match i {
            1 => "a",
            2 => "b",
            3 => "c",
            4 => "d",
            5 => "e",
            6 => "f",
            7 => "g",
            8 => "h",
            9 => "i",
            _ => "",
        }
        .to_string()
}

pub fn enc_move(m: &Move) -> String {
    let mut ret = String::new();
    match m.from {
        Some(p) => write!(&mut ret, "{}{}", p.x, dan(p.y)).unwrap(),
        None => write!(&mut ret, "*").unwrap(),
    }
    write!(&mut ret, "{}{}", m.to.x, dan(m.to.y)).unwrap();
    ret
}

pub fn position(p: &Position, moves: &Vec<Move>) -> String {
    let mut ret = String::new();
    write!(ret, "position sfen {} moves", sfen(p)).unwrap();
    for &m in moves {
        write!(ret, " {}", enc_move(&m)).unwrap()
    }
    ret
}

#[cfg(test)]
mod tests {
    use types::*;
    use std::collections::BTreeMap;

    #[test]
    fn it_works() {
        let mut captured = BTreeMap::new();
        assert_eq!(super::captured(&captured), "-");

        captured.insert((Color::Black, Piece::Silver), 1);
        captured.insert((Color::Black, Piece::Pawn), 2);
        captured.insert((Color::White, Piece::Bishop), 1);
        captured.insert((Color::White, Piece::Pawn), 3);

        assert_eq!(super::captured(&captured), "2PS3pb");
        assert_eq!(super::board(&hirate_board()),
                   "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL");

        captured.insert((Color::White, Piece::Pawn), 3);

        assert_eq!(super::enc_move(&Move {
                       c: Color::Black,
                       from: Some(Point::new(7, 7)),
                       to: Point::new(7, 6),
                       p: Piece::Pawn,
                   }),
                   "7g7f");
        assert_eq!(super::enc_move(&Move {
                       c: Color::Black,
                       from: None,
                       to: Point::new(7, 6),
                       p: Piece::Pawn,
                   }),
                   "*7f");

        assert_eq!(super::position(&Position {
                                       board: hirate_board(),
                                       captured: BTreeMap::new(),
                                       c: Color::Black,
                                   },
                                   &vec![Move {
                                             c: Color::Black,
                                             from: Some(Point::new(7, 7)),
                                             to: Point::new(7, 6),
                                             p: Piece::Pawn,
                                         },
                                         Move {
                                             c: Color::White,
                                             from: Some(Point::new(3, 3)),
                                             to: Point::new(3, 4),
                                             p: Piece::Pawn,
                                         }]),
                   "position sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 \
                    moves 7g7f 3c3d")

    }
}
