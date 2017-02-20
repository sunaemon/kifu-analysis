use types::*;
use std::fmt::Write;

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
            match b[Point::new(j, i)] {
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

pub fn captured(captured: &Captured) -> String {
    let mut ret = String::new();
    if captured.is_empty() {
        write!(ret, "-").unwrap();
    }
    for (&(c, p), &n) in captured.into_iter() {
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
           board(p.board()),
           match p.color() {
               Color::Black => "b",
               Color::White => "w",
           },
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
        Some(p) => write!(&mut ret, "{}{}", p.x, dan(p.y)).unwrap(),
        None => write!(&mut ret, "*").unwrap(),
    }
    write!(&mut ret, "{}{}", m.to().x, dan(m.to().y)).unwrap();
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

    #[test]
    fn it_works() {
        let mut captured = Captured::new();
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

        assert_eq!(super::enc_move(&Move::new(Color::Black,
                                              Some(Point::new(7, 7)),
                                              Point::new(7, 6),
                                              Piece::Pawn,
                                              false)
                       .unwrap()),
                   "7g7f");
        assert_eq!(super::enc_move(&Move::new(Color::Black,
                                              None,
                                              Point::new(7, 6),
                                              Piece::Pawn,
                                              false)
                       .unwrap()),
                   "*7f");

        assert_eq!(super::position(&Position::new(Board::hirate(), Captured::new(), Color::Black),
                                   &vec![Move::new(Color::Black,
                                                   Some(Point::new(7, 7)),
                                                   Point::new(7, 6),
                                                   Piece::Pawn,
                                                   false)
                                             .unwrap(),
                                         Move::new(Color::White,
                                                   Some(Point::new(3, 3)),
                                                   Point::new(3, 4),
                                                   Piece::Pawn,
                                                   false)
                                             .unwrap()]),
                   "position sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 \
                    moves 7g7f 3c3d")

    }


    #[test]
    fn make_move_check() {
        let mut p = Position::hirate();
        p.make_move(&Move::new(Color::Black,
                                  Some(Point::one_start(7, 7)),
                                  Point::one_start(7, 6),
                                  Piece::Pawn,
                                  false)
                .unwrap())
            .unwrap();

        assert_eq!(super::sfen(&p),
                   "lnsgkgsnl/1r5b1/ppppppppp/9/9/6P2/PPPPPP1PP/1B5R1/LNSGKGSNL w - 1");

        let mut p = Position::hirate();
        let m = Move::new(Color::Black,
                          Some(Point::one_start(7, 7)),
                          Point::one_start(7, 6),
                          Piece::PPawn,
                          true)
            .unwrap();
        p.make_move(&m).unwrap();

        assert_eq!(super::sfen(&p),
                   "lnsgkgsnl/1r5b1/ppppppppp/9/9/6+P2/PPPPPP1PP/1B5R1/LNSGKGSNL w - 1");
    }
}
