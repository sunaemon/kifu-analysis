use types::*;
use types;
use combine::{self, many, Parser, one_of, satisfy_map, token, optional, sep_by, Stream, ParseResult,
              parser, none_of, try, value, ConsumedResult, tokens, ParseError};
use combine::byte::digit;
use std::str::FromStr;
use std::str::from_utf8;
use std::marker::PhantomData;
use std::io::BufRead;

fn piece_with_color_parser<I>(input: I) -> ParseResult<(Color, Piece), I>
    where I: Stream<Item = u8>
{
    (optional(token(b'+')),
     satisfy_map(|c| match c {
         b'P' => Some((Color::Black, Piece::Pawn)),
         b'L' => Some((Color::Black, Piece::Lance)),
         b'N' => Some((Color::Black, Piece::Knight)),
         b'S' => Some((Color::Black, Piece::Silver)),
         b'G' => Some((Color::Black, Piece::Gold)),
         b'B' => Some((Color::Black, Piece::Bishop)),
         b'R' => Some((Color::Black, Piece::Rook)),
         b'K' => Some((Color::Black, Piece::King)),
         b'p' => Some((Color::White, Piece::Pawn)),
         b'l' => Some((Color::White, Piece::Lance)),
         b'n' => Some((Color::White, Piece::Knight)),
         b's' => Some((Color::White, Piece::Silver)),
         b'g' => Some((Color::White, Piece::Gold)),
         b'b' => Some((Color::White, Piece::Bishop)),
         b'r' => Some((Color::White, Piece::Rook)),
         b'k' => Some((Color::White, Piece::King)),
         _ => None,
     }))
        .map(|t| {
            let (promoted, (c, p)) = t;
            (c,
             if promoted.is_some() {
                 p.promote().unwrap()
             } else {
                 p
             })
        })
        .parse_stream(input)
}

#[allow(dead_code)]
fn board_parser<I>(input: I) -> ParseResult<Option<Board>, I>
    where I: Stream<Item = u8>
{
    enum PosisionPaserTemp {
        PieceWithColor(Color, Piece),
        Blank(u8),
    }

    let digit_as_u8 = one_of(b"123456789".to_vec()).map(|d| d - b'0');

    sep_by(many(choice!(parser(piece_with_color_parser)
                            .map(|(c, p)| PosisionPaserTemp::PieceWithColor(c, p)),
                        digit_as_u8.map(|n| PosisionPaserTemp::Blank(n))))
               .map(|d: Vec<PosisionPaserTemp>| {
            d.iter().fold(Vec::new(), move |mut a, d| {
                match d {
                    &PosisionPaserTemp::PieceWithColor(c, p) => a.push(Some((c, p))),
                    &PosisionPaserTemp::Blank(n) => {
                        for _ in 0..n {
                            a.push(None)
                        }
                    }
                }
                a
            })
        }),
           token(b'/'))
        .map(|b: Vec<Vec<Option<(Color, Piece)>>>| {
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

            Some(types::Board::new(ret))
        })
        .parse_stream(input)
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord, RustcDecodable, RustcEncodable)]
pub enum Score {
    Cp(i64),
    Mate(i64),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord, RustcDecodable, RustcEncodable)]
pub enum PrivimiveMove {
    Drop { c: Color, p: Piece, to: Point },
    Move {
        from: Point,
        to: Point,
        promotion: bool,
    },
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord, RustcDecodable, RustcEncodable)]
pub enum Info {
    String(Vec<u8>),
    Depth(u64),
    SelDepth(u64),
    Time(u64),
    Nodes(u64),
    NodesPerSecond(u64),
    HashFull(u64),
    Score(Score),
    MultiPv(u64),
    Pv(Vec<PrivimiveMove>),
}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord, RustcDecodable, RustcEncodable)]
pub enum Response {
    UsiOk,
    ReadyOk,
    BestMove(PrivimiveMove),
    Infos(Vec<Info>),
}

//hashfull 11 score cp 0 multipv 1 pv 9d9c 5d5f 4g5f 1d1e 1f1e \
//1a1e 1i1e 2c2d 2e2d 3c2d 1e1a+ B*6e 5f6e"),

fn point_parser<I>(input: I) -> ParseResult<Point, I>
    where I: Stream<Item = u8>
{

    let digit_as_u8 = one_of(b"123456789".to_vec()).map(|d| d - b'0');
    let alphabet_as_u8 = one_of(b"abcdefghi".to_vec()).map(|d| d - b'a' + 1);
    (digit_as_u8, alphabet_as_u8)
        .map(|(x, y)| Point::new(x - 1, y - 1))
        .parse_stream(input)
}

fn pmove_parser<I>(input: I) -> ParseResult<PrivimiveMove, I>
    where I: Stream<Item = u8>
{
    choice!((parser(piece_with_color_parser).skip(token(b'*')), parser(point_parser))
                .map(|((c, p), to)| {
            PrivimiveMove::Drop {
                c: c,
                p: p,
                to: to,
            }
        }),
            (parser(point_parser), parser(point_parser), optional(token(b'+')))
                .map(|(from, to, promotion)| {
            PrivimiveMove::Move {
                from: from,
                to: to,
                promotion: promotion.is_some(),
            }
        }))
        .parse_stream(input)
}

fn uint_parser<I>(input: I) -> ParseResult<u64, I>
    where I: Stream<Item = u8>
{
    many(digit())
        .map(|ds: Vec<u8>| u64::from_str(from_utf8(&ds).unwrap()).unwrap())
        .parse_stream(input)
}

fn int_parser<I>(input: I) -> ParseResult<i64, I>
    where I: Stream<Item = u8>
{
    (optional(token(b'-')), parser(uint_parser))
        .map(|(m, n)| if m.is_none() { n as i64 } else { -(n as i64) })
        .parse_stream(input)
}
fn eq(l: u8, r: u8) -> bool {
    l == r
}
#[derive(Clone)]
pub struct Bytes<I>(&'static [u8], PhantomData<fn(I) -> I>) where I: Stream<Item = u8>;
impl<I> Parser for Bytes<I>
    where I: Stream<Item = u8>
{
    type Input = I;
    type Output = &'static [u8];
    #[inline]
    fn parse_lazy(&mut self, input: Self::Input) -> ConsumedResult<Self::Output, Self::Input> {
        tokens(eq,
               combine::primitives::Info::Owned(from_utf8(self.0).unwrap().to_string()),
               self.0.to_vec())
            .parse_lazy(input)
            .map(|_| self.0)
    }
    fn add_error(&mut self, errors: &mut ParseError<Self::Input>) {
        tokens(eq,
               combine::primitives::Info::Owned(from_utf8(self.0).unwrap().to_string()),
               self.0.to_vec())
            .add_error(errors)
    }
}

fn bytes<I>(b: &'static [u8]) -> Bytes<I>
    where I: Stream<Item = u8>
{
    Bytes(b, PhantomData)
}

fn info_parser<I>(input: I) -> ParseResult<Info, I>
    where I: Stream<Item = u8>
{
    choice!(bytes(b"pv ")
                .with(sep_by(parser(pmove_parser), token(b' ')))
                .map(|pv| Info::Pv(pv)),
            try(bytes(b"nps ")).with(parser(uint_parser).map(|n| Info::NodesPerSecond(n))),
            bytes(b"time ").with(parser(uint_parser).map(|n| Info::Time(n))),
            bytes(b"nodes ").with(parser(uint_parser).map(|n| Info::Nodes(n))),
            bytes(b"depth ").with(parser(uint_parser).map(|n| Info::Depth(n))),
            try(bytes(b"score "))
                .with(choice!(bytes(b"cp ").with(parser(int_parser).map(|i| Score::Cp(i))),
                              bytes(b"mate ").with(parser(int_parser).map(|i| Score::Mate(i))))
                    .map(|s| Info::Score(s))),
            bytes(b"hashfull ").with(parser(uint_parser).map(|n| Info::HashFull(n))),
            try(bytes(b"seldepth ")).with(parser(uint_parser).map(|n| Info::SelDepth(n))),
            bytes(b"multipv ").with(parser(uint_parser).map(|mpv| Info::MultiPv(mpv))),
            bytes(b"string ").with(many(none_of(b"\n".to_vec())).map(|mpv| Info::String(mpv))))
        .parse_stream(input)
}

fn response_parser<I>(input: I) -> ParseResult<Response, I>
    where I: Stream<Item = u8>
{
    choice!(bytes(b"info ")
                .with(sep_by(parser(info_parser), token(b' ')).map(|infos| Response::Infos(infos))),
            bytes(b"usiok").with(value(Response::UsiOk)),
            bytes(b"readyok").with(value(Response::ReadyOk)),
            bytes(b"bestmove ").with(parser(pmove_parser)).map(|s| Response::BestMove(s)))
        .parse_stream(input)
}


pub fn read_and_parse<R, F, S>(reader: &mut R, mut callback: F) -> S
    where F: FnMut(Response) -> Option<S>,
          R: BufRead
{
    let mut line = String::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        debug!("Read {:?}", line);

        match parser(response_parser).parse(line.as_bytes()) {
            Ok((res, _)) => {
                if let Some(d) = callback(res) {
                    return d;
                }
            }
            Err(e) => info!("parse error{:?}", e),
        }
        line.clear();
    }

    panic!("exit");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(parser(piece_with_color_parser).parse(&b"+b"[..]),
                   Ok(((Color::White, Piece::Horse), &b""[..])));

        assert_eq!(parser(board_parser)
                       .parse(&b"lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL"[..]),
                   Ok((Some(types::Board::hirate()), &b""[..])));


        use types::Color::*;
        use types::Piece::*;
        use super::PrivimiveMove::*;

        let moves = vec![Move {
                             from: Point { x: 8, y: 3 },
                             to: Point { x: 8, y: 2 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 4, y: 3 },
                             to: Point { x: 4, y: 5 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 3, y: 6 },
                             to: Point { x: 4, y: 5 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 0, y: 3 },
                             to: Point { x: 0, y: 4 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 0, y: 5 },
                             to: Point { x: 0, y: 4 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 0, y: 0 },
                             to: Point { x: 0, y: 4 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 0, y: 8 },
                             to: Point { x: 0, y: 4 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 1, y: 2 },
                             to: Point { x: 1, y: 3 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 1, y: 4 },
                             to: Point { x: 1, y: 3 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 2, y: 2 },
                             to: Point { x: 1, y: 3 },
                             promotion: false,
                         },
                         Move {
                             from: Point { x: 0, y: 4 },
                             to: Point { x: 0, y: 0 },
                             promotion: true,
                         },
                         Drop {
                             c: Black,
                             p: Bishop,
                             to: Point { x: 5, y: 4 },
                         },
                         Move {
                             from: Point { x: 4, y: 5 },
                             to: Point { x: 5, y: 4 },
                             promotion: false,
                         }];

        assert_eq!(parser(response_parser)
                       .parse(&b"info depth 13 seldepth 13 time 132 nodes 1129633 nps 8557825 \
                               hashfull 11 score cp 0 multipv 1 pv 9d9c 5d5f 4g5f 1d1e 1f1e \
                               1a1e 1i1e 2c2d 2e2d 3c2d 1e1a+ B*6e 5f6e"
                                   [..]),
                   Ok((Response::Infos(vec![Info::Depth(13),
                                            Info::SelDepth(13),
                                            Info::Time(132),
                                            Info::Nodes(1129633),
                                            Info::NodesPerSecond(8557825),
                                            Info::HashFull(11),
                                            Info::Score(Score::Cp(0)),
                                            Info::MultiPv(1),
                                            Info::Pv(moves)]),
                       &b""[..])));
    }
}
