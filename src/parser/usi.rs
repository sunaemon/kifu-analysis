use types::*;
use nom::{self, space, digit, IResult, AsChar};
use std::str::from_utf8;
use std::collections::VecDeque;
use std::io::Read;
use std::ops::{Range, RangeFrom, RangeTo};
use nom::IResult::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PrivimiveMove {
    pub c: Color,
    pub from: Point,
    pub to: Point,
    pub promote: bool,
}

pub fn minus_or_digit<T>(input: T) -> IResult<T, T>
    where T: nom::Slice<Range<usize>> + nom::Slice<RangeFrom<usize>> + nom::Slice<RangeTo<usize>>,
          T: nom::InputIter + nom::InputLength
{
    let input_length = input.input_len();
    if input_length == 0 {
        return Incomplete(nom::Needed::Unknown);
    }

    for (idx, item) in input.iter_indices() {
        let c = item.as_char();
        if !(c == '-' || (c >= '0' && c <= '9')) {
            if idx == 0 {
                return Error(error_position!(nom::ErrorKind::Digit, input));
            } else {
                return Done(input.slice(idx..), input.slice(0..idx));
            }
        }
    }
    Done(input.slice(input_length..), input)
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

named_attr!(#[allow(dead_code)], point<&[u8], Point>,
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
            ret[i][j] = b[i][8 - j]
        }
    }

    Some(Board::new(ret))
}

named_attr!(#[allow(dead_code)], board_parse<&[u8],Option<Board> >,
       map!(board_parse_vec,
            vec_board_to_board));

fn not_newline(c: u8) -> bool {
    c != b'\n'
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub enum Score {
    Cp(i64),
    Mate(i64),
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub enum Info {
    String(String),
    Depth(u64),
    SelDepth(u64),
    Time(u64),
    Nodes(u64),
    NodesPerSecond(u64),
    HashFull(u64),
    Score(Score),
    MultiPv(u64),
    Pv(String),
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub enum Response {
    UsiOk,
    ReadyOk,
    BestMove(String),
    Infos(Vec<Info>),
}

fn atou(d: &[u8]) -> u64 {
    from_utf8(d).unwrap().parse().unwrap()
}
fn atoi(d: &[u8]) -> i64 {
    from_utf8(d).unwrap().parse().unwrap()
}

named!(info_internal<&[u8], Info>,
       preceded!(
         opt!(space),
       alt!(
         chain!(
           tag!("pv ") ~
           ret: take_while!(not_newline),
           || Info::Pv(from_utf8(ret).unwrap().to_string())
           ) |
         chain!(
           tag!("nps ") ~
           d: digit,
           || Info::NodesPerSecond(atou(d))
           ) |
         chain!(
           tag!("time ") ~
           d: digit,
           || Info::Time(atou(d))
           ) |
         chain!(
           tag!("nodes ") ~
           d: digit,
           || Info::Nodes(atou(d))
           ) |
         chain!(
           tag!("depth ") ~
           d: digit,
           || Info::Depth(atou(d))
           ) |
         chain!(
           tag!("score ") ~
           s: alt!(
             chain!(tag!("cp ") ~ d: minus_or_digit,  || Score::Cp(atoi(d))) |
             chain!(tag!("mate ") ~ d: minus_or_digit,  || Score::Mate(atoi(&d)))),
           || Info::Score(s)
           ) |
         chain!(
           tag!("hashfull ") ~
           d: digit,
           || Info::HashFull(atou(d))
           ) |
         chain!(
           tag!("seldepth ") ~
           d: digit,
           || Info::SelDepth(atou(d))
           ) |
         chain!(
           tag!("multipv ") ~
           d: digit,
           || Info::MultiPv(atou(d))
           ) |
         chain!(
           tag!("string ") ~
           ret: take_while!(not_newline),
           || Info::String(from_utf8(ret).unwrap().to_string())
           )
       )));

named!(info<&[u8], Response>,
       chain!(
       tag!(b"info") ~
       infos: many0!(info_internal),
       || Response::Infos(infos)
       ));

named!(pub response<&[u8], Response>,
       preceded!(
         opt!(tag!("\n")),
         alt!(
           tag!(b"usiok\n") => { |_| Response::UsiOk } |
           tag!(b"readyok\n") => { |_| Response::ReadyOk } |
           chain!(tag!(b"bestmove") ~
                  ret: take_while!(not_newline),
                  || Response::BestMove(from_utf8(ret).unwrap().to_string()) ) |
           info)));

pub fn read_and_parse<F, T, R>(data: &mut T, mut callback: F) -> R
    where F: FnMut(Response) -> Option<R>,
          T: Read
{
    let mut q = VecDeque::new();
    loop {
        let mut buf = [0u8; 4096];
        let n = data.read(&mut buf).unwrap();
        debug!("Read {:?}", from_utf8(&buf[0..n]).unwrap());

        for i in 0..n {
            q.push_back(buf[i]);
        }

        loop {
            let b: Vec<_> = q.iter().cloned().collect(); // This copy can be avoided by using combine.

            match response(b.as_slice()) {
                IResult::Done(rest, r) => {
                    debug!("Done {:?}", r);

                    for _ in 0..(q.len() - rest.len()) {
                        q.pop_front();
                    }
                    if let Some(d) = callback(r) {
                        debug!("rest: {:?}",
                               from_utf8(q.iter().cloned().collect::<Vec<_>>().as_slice())
                                   .unwrap());
                        return d;
                    }
                }
                IResult::Incomplete(n) => {
                    debug!("Incomplete {:?}", n);
                    break;
                }
                IResult::Error(e) => debug!("Err {}", e),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use nom::IResult;

    use types::*;
    use super::*;

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

    #[test]
    fn usi_conv() {
        assert_eq!(super::info_internal(b"depth 1 seldepth 11"),
                   IResult::Done(&b" seldepth 11"[..], Info::Depth(1)));
        assert_eq!(super::info(b"info depth 1 seldepth 11"),
                   IResult::Done(&b""[..],
                                 Response::Infos(vec![Info::Depth(1), Info::SelDepth(11)])));

        assert_eq!(super::info(b"info depth 13 seldepth 13 time 132 nodes 1129633 nps 8557825 hashfull 11 score cp 0 \
                                 multipv 1 pv 9d9c 5d5f 4g5f 1d1e 1f1e 1a1e 1i1e 2c2d 2e2d 3c2d 1e1a+ B*6e 5f6e\n"),
                   IResult::Done(&b"\n"[..], Response::Infos(vec! [Info::Depth(13), Info::SelDepth(13), Info::Time(132), Info::Nodes(1129633), Info::NodesPerSecond(8557825),
                   Info::HashFull(11), Info::Score(Score::Cp(0)), Info::MultiPv(1), Info::Pv("9d9c 5d5f 4g5f 1d1e 1f1e 1a1e 1i1e 2c2d 2e2d 3c2d 1e1a+ B*6e 5f6e".to_string())])));
    }
}
