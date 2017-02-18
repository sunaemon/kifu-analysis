use nom::{IError, digit};
use types::*;

named!(digit_as_u8<&[u8], u8>,
       map!(one_of!("0123456789"), |d:char| d.to_digit(10).unwrap() as u8));

named!(piece<&[u8], Piece>,
       alt!(
         tag!("FU") => { |_| Piece::Pawn } |
         tag!("KY") => { |_| Piece::Lance } |
         tag!("KE") => { |_| Piece::Knight } |
         tag!("GI") => { |_| Piece::Silver } |
         tag!("KI") => { |_| Piece::Gold } |
         tag!("KA") => { |_| Piece::Bishop } |
         tag!("HI") => { |_| Piece::Rook } |
         tag!("OU") => { |_| Piece::King } |
         tag!("TO") => { |_| Piece::PPawn } |
         tag!("NY") => { |_| Piece::PLance } |
         tag!("NK") => { |_| Piece::PKnight } |
         tag!("NG") => { |_| Piece::PSilver } |
         tag!("UM") => { |_| Piece::Horse } |
         tag!("RY") => { |_| Piece::Dragon }));

named!(color<&[u8], Color>,
       alt!(
         tag!("+") => { |_| Color::Black } |
         tag!("-") => { |_| Color::White }));

named!(point<&[u8], Point>,
       chain!(
         x: digit_as_u8 ~
         y: digit_as_u8,
         || Point::new(x, y)));

named!(parse_move<&[u8], Move>,
       chain!(
         c: color ~
         from: point  ~
         to: point ~
         p: piece ~
         tag!(",") ~
         tag!("L") ~
         rest_time: digit,
         || Move {
              c: c,
              from: from,
              to: to,
              p: p
         }));

named!(parse_issue_of_game<&[u8], IssueOfGame>,
       alt!(
         chain!(
           c: alt!(
             tag!("GOTE_") => { |_| Color::White } |
             tag!("SENTE_") => { |_| Color::Black }) ~
           tag!("WIN_") ~
           w: alt!(
             tag!("TORYO") => { |_| Win::Toryo } |
             tag!("TIMEOUT") => { |_| Win::Timeout } |
             tag!("CHECKMATE") => { |_| Win::Checkmate } |
             tag!("ENTERINGKING") => { |_| Win::EnteringKing } |
             tag!("OUTE_SENNICHI") => { |_| Win::OuteSennnichi }),
             || IssueOfGame::Win(c,w) ) |
         tag!("DRAW_SENNICHI") => { |_| IssueOfGame::Draw(Draw::Sennnichi) }));

named!(parse_step<&[u8], Step>,
       preceded!(
         opt!(tag!("\t")),
         alt!(
           parse_move => { |m| Step::Move(m) } |
           parse_issue_of_game => { |i| Step::IssueOfGame(i) })));

named!(parse_kifu<&[u8], Vec<Step> >,
       many0!(parse_step));

pub fn parse(s: &[u8]) -> Result<Vec<Step>, IError<u32>> {
    parse_kifu(s).to_full_result()
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::parse_move;
    use super::parse_issue_of_game;
    use super::parse_step;
    use super::parse_kifu;
    use types::*;

    #[test]
    fn it_works() {
        assert_eq!(parse_move(b"+7776FU,L599"),
                   IResult::Done(&b""[..],
                                 Move {
                                     c: Color::Black,
                                     from: Point::new(7, 7),
                                     to: Point::new(7, 6),
                                     p: Piece::Pawn,
                                 }));
        assert_eq!(parse_issue_of_game(b"GOTE_WIN_TORYO"),
                   IResult::Done(&b""[..], IssueOfGame::Win(Color::White, Win::Toryo)));
        assert_eq!(parse_step(b"	+7776FU,L599	-3334FU,L599	GOTE_WIN_TORYO"),
                   IResult::Done(&b"	-3334FU,L599	GOTE_WIN_TORYO"[..],
                                 Step::Move(Move {
                                     c: Color::Black,
                                     from: Point::new(7, 7),
                                     to: Point::new(7, 6),
                                     p: Piece::Pawn,
                                 })));
        assert_eq!(parse_step(b"GOTE_WIN_TORYO"),
                   IResult::Done(&b""[..],
                                 Step::IssueOfGame(IssueOfGame::Win(Color::White, Win::Toryo))));
        assert_eq!(parse_kifu(b"+7776FU,L599	-3334FU,L599	GOTE_WIN_TORYO"),
                   IResult::Done(&b""[..],
                                 vec![Step::Move(Move {
                                          c: Color::Black,
                                          from: Point::new(7, 7),
                                          to: Point::new(7, 6),
                                          p: Piece::Pawn,
                                      }),
                                      Step::Move(Move {
                                          c: Color::White,
                                          from: Point::new(3, 3),
                                          to: Point::new(3, 4),
                                          p: Piece::Pawn,
                                      }),
                                      Step::IssueOfGame(IssueOfGame::Win(Color::White,
                                                                         Win::Toryo))]));
    }
}
