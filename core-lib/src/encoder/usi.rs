use std::fmt::Write;
use types::*;

pub fn color(c: Color) -> String {
    match c {
            Color::Black => "b",
            Color::White => "w",
        }
        .to_string()
}

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
    for j in 1..10 {
        for i in (1..10).rev() {
            match b[Point::one_start(i, j)] {
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
        if j != 9 {
            write!(ret, "/").unwrap();
        }
    }
    ret
}

pub fn captured(captured: &Captured) -> String {
    let mut ret = String::new();
    if captured.is_empty() {
        write!(ret, "-").unwrap();
    }
    for (&c, cc) in captured.to_inner() {
        for (&p, &n) in cc {
            if n == 1 {
                write!(ret, "{}", piece_with_color(c, p)).unwrap();
            } else if n > 1 {
                write!(ret, "{}{}", n, piece_with_color(c, p)).unwrap();
            }
        }
    }
    ret
}

pub fn sfen(p: &Position) -> String {
    let mut ret = String::new();
    write!(&mut ret,
           "{} {} {} 1",
           board(p.board()),
           color(p.color()),
           captured(p.captured()))
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
    match m.from() {
        Some(p) => write!(ret, "{}{}", p.x + 1, dan(p.y + 1)).unwrap(),
        None => write!(ret, "{}*", piece_with_color(m.color(), m.piece())).unwrap(),
    }
    write!(ret, "{}{}", m.to().x + 1, dan(m.to().y + 1)).unwrap();
    if m.is_promote() {
        write!(ret, "+").unwrap()
    }
    ret
}

pub fn position(p: &Position, moves: &[Move]) -> String {
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

    #[test]
    fn it_works() {
        let mut captured = Captured::default();
        assert_eq!(super::captured(&captured), "-");

        captured.add(Color::Black, Piece::Silver);
        captured.add(Color::Black, Piece::Pawn);
        captured.add(Color::Black, Piece::Pawn);
        captured.add(Color::White, Piece::Bishop);
        captured.add(Color::White, Piece::Pawn);
        captured.add(Color::White, Piece::Pawn);
        captured.add(Color::White, Piece::Pawn);

        assert_eq!(super::captured(&captured), "2PS3pb");
        assert_eq!(super::board(&Board::hirate()),
                   "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL");

        assert_eq!(super::enc_move(&Move::Move {
                       color: Color::Black,
                       from: Point::one_start(7, 7),
                       to: Point::one_start(7, 6),
                       piece: Piece::Pawn,
                       promote: false,
                   }),
                   "7g7f");
        assert_eq!(super::enc_move(&Move::Drop {
                       color: Color::Black,
                       to: Point::one_start(7, 6),
                       piece: Piece::Pawn,
                   }),
                   "P*7f");

        assert_eq!(super::position(&Position::new(Board::hirate(),
                                                  Captured::default(),
                                                  Color::Black),
                                   &vec![Move::Move {
                                             color: Color::Black,
                                             from: Point::one_start(7, 7),
                                             to: Point::one_start(7, 6),
                                             piece: Piece::Pawn,
                                             promote: false,
                                         },
                                         Move::Move {
                                             color: Color::White,
                                             from: Point::one_start(3, 3),
                                             to: Point::one_start(3, 4),
                                             piece: Piece::Pawn,
                                             promote: false,
                                         }]),
                   "position sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 \
                    moves 7g7f 3c3d")

    }

    #[test]
    fn make_move_check() {
        let mut p = Position::hirate();
        p.make_move(&Move::Move {
                color: Color::Black,
                from: Point::one_start(7, 7),
                to: Point::one_start(7, 6),
                piece: Piece::Pawn,
                promote: false,
            })
            .unwrap();

        assert_eq!(super::sfen(&p),
                   "lnsgkgsnl/1r5b1/ppppppppp/9/9/2P6/PP1PPPPPP/1B5R1/LNSGKGSNL w - 1");

        let mut p = Position::hirate();
        let m = Move::Move {
            color: Color::Black,
            from: Point::one_start(7, 7),
            to: Point::one_start(7, 6),
            piece: Piece::PPawn,
            promote: true,
        };
        p.make_move(&m).unwrap();

        assert_eq!(super::sfen(&p),
                   "lnsgkgsnl/1r5b1/ppppppppp/9/9/2+P6/PP1PPPPPP/1B5R1/LNSGKGSNL w - 1");
    }
}
