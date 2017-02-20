use types::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PrivimiveMove {
    pub c: Color,
    pub from: Point,
    pub to: Point,
    pub promote: bool,
}

named!(digit_as_u8<&[u8], u8>,
       map!(one_of_bytes!(b"123456789"), |d:char| d.to_digit(10).unwrap() as u8));
named!(alphabet_as_u8<&[u8], u8>,
       map!(one_of_bytes!(b"abcdefghi"), |d:char| (char::to_string(&d).as_bytes()[0] - b'a' + 1)));

named!(piece_without_promotion_with_color<&[u8], (Color, Piece)>,
       alt!(
         tag!("P") => { |_| (Color::Black, Piece::Pawn) } |
         tag!("L") => { |_| (Color::Black, Piece::Lance) } |
         tag!("N") => { |_| (Color::Black, Piece::Knight) } |
         tag!("S") => { |_| (Color::Black, Piece::Silver) } |
         tag!("G") => { |_| (Color::Black, Piece::Gold) } |
         tag!("B") => { |_| (Color::Black, Piece::Bishop) } |
         tag!("R") => { |_| (Color::Black, Piece::Rook) } |
         tag!("K") => { |_| (Color::Black, Piece::King) } |
         tag!("p") => { |_| (Color::White, Piece::Pawn) } |
         tag!("l") => { |_| (Color::White, Piece::Lance) } |
         tag!("n") => { |_| (Color::White, Piece::Knight) } |
         tag!("s") => { |_| (Color::White, Piece::Silver) } |
         tag!("g") => { |_| (Color::White, Piece::Gold) } |
         tag!("b") => { |_| (Color::White, Piece::Bishop) } |
         tag!("r") => { |_| (Color::White, Piece::Rook) } |
         tag!("k") => { |_| (Color::White, Piece::King) }));

named!(piece_with_color<&[u8], (Color, Piece)>,
       chain!(
         promoted: tag!("+")? ~
         c_p: piece_without_promotion_with_color,
         || match c_p {
           (c,p) => (c, if promoted.is_some() {p.promote().unwrap()} else {p})
         }));

named!(point<&[u8], Point>,
       chain!(
         x: digit_as_u8 ~
         y: alphabet_as_u8,
         || Point::new(x, y)));

enum PosisionPaserTemp {
    PieceWithColor(Color, Piece),
    Blank(u8),
}

named!(board_line_parse_sub(&[u8]) -> PosisionPaserTemp,
       alt!(
         piece_with_color => { |(c,p)| PosisionPaserTemp::PieceWithColor(c,p) } |
         digit_as_u8 => { | n | PosisionPaserTemp::Blank(n) }));

named!(board_line_parse(&[u8]) -> Vec<Option<(Color, Piece)>>,
       preceded!(
         opt!(tag!("/")),
         fold_many0!(
           board_line_parse_sub,
           Vec::new(),
           | mut acc: Vec<_>, t | {
             match t {
               PosisionPaserTemp::PieceWithColor(c,p) => acc.push(Some((c,p))),
               PosisionPaserTemp::Blank(n) => for i in 0 .. n { acc.push(None) }
             }
             acc })));

type VecBoard = Vec<Vec<Option<(Color, Piece)>>>;

named!(board_parse_vec(&[u8]) -> VecBoard,
       dbg!(many0!(board_line_parse)));

fn vec_board_to_board(b: VecBoard) -> Option<Board> {
    let mut ret: [[Option<(Color, Piece)>; 9]; 9] = [[None; 9]; 9];

    if b.len() != 9 {
        return None;
    }

    for i in 0..9 {
        if b[i].len() != 9 {
            return None;
        }
        for j in 0..9 {
            ret[i][j] = b[i][j]
        }
    }

    Some(Board::new(ret))
}

named!(board_parse<&[u8],Option<Board> >,
       map!(board_parse_vec,
            vec_board_to_board));

#[cfg(test)]
mod tests {
    use nom::IResult;

    use types::*;

    #[test]
    fn it_works() {
        assert_eq!(super::piece_with_color(b"+b"),
                   IResult::Done(&b""[..], (Color::White, Piece::Horse)));
        assert_eq!(super::alphabet_as_u8(b"a"), IResult::Done(&b""[..], 1));
        assert_eq!(super::alphabet_as_u8(b"b"), IResult::Done(&b""[..], 2));
        assert_eq!(super::alphabet_as_u8(b"i"), IResult::Done(&b""[..], 9));
        assert_eq!(super::board_line_parse(b"1r5b1"),
                   IResult::Done(&b""[..],
                                 vec![None,
                                      Some((Color::White, Piece::Rook)),
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      Some((Color::White, Piece::Bishop)),
                                      None]));
        assert_eq!(super::board_parse_vec(b"1r5b1/9"),
                   IResult::Done(&b""[..],
                                 vec![vec![None,
                                           Some((Color::White, Piece::Rook)),
                                           None,
                                           None,
                                           None,
                                           None,
                                           None,
                                           Some((Color::White, Piece::Bishop)),
                                           None],
                                      vec![None, None, None, None, None, None, None, None, None]]));

        assert_eq!(super::board_parse(b"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL"),
                   IResult::Done(&b""[..], Some(Board::hirate())));
    }
}
