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

named!(point<&[u8], Option<Point>>,
       chain!(
         x: digit_as_u8 ~
         y: digit_as_u8,
         || if (x,y) != (0,0) {
           Some(Point::one_start(x, y))
         } else {
           None
         }
             ));

#[derive(PartialEq, Copy, Clone, Debug)]
struct PrimitiveMove {
    color: Color,
    from: Option<Point>,
    to: Point,
    piece: Piece,
}

impl PrimitiveMove {
    fn new(color: Color, from: Option<Point>, to: Point, p: Piece) -> PrimitiveMove {
        PrimitiveMove {
            color: color,
            from: from,
            to: to,
            piece: p,
        }
    }
    fn to_move(&self, promote: bool) -> Move {
        Move::new(self.color, self.from, self.to, self.piece, promote).unwrap()
    }

    fn primive_move_to_move(&self, p: &Position) -> Move {
        if let Some(from) = self.from {
            if p.board()[from] != Some((self.color, self.piece)) {
                return self.to_move(true);
            }
        }
        self.to_move(false)
    }
}



named!(parse_move<&[u8], PrimitiveMove>,
       chain!(
         c: color ~
         from: point  ~
         to: point ~
         p: piece ~
         tag!(",") ~
         tag!("L") ~
         rest_time: digit,
         || PrimitiveMove::new (
              c,
              from,
              to.unwrap(),
              p,
         )));

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



#[derive(PartialEq, Copy, Clone, Debug)]
enum Step {
    Move(PrimitiveMove),
    IssueOfGame(IssueOfGame),
}

named!(parse_step<&[u8], Step>,
       preceded!(
         opt!(tag!("\t")),
         alt!(
           parse_move => { |m| Step::Move(m) } |
           parse_issue_of_game => { |i| Step::IssueOfGame(i) })));

named!(parse_kifu_primitive<&[u8], Vec<Step> >,
       many0!(parse_step));

fn parse_primitive(s: &[u8]) -> Result<Vec<Step>, IError<u32>> {
    parse_kifu_primitive(s).to_full_result()
}

pub fn parse(s: &[u8]) -> Result<Game, String> {
    let mut g = Game::hirate();
    let mut p = g.position.clone();
    for s in parse_primitive(s).or(Err("Parse Error"))? {
        match s {
            Step::Move(pm) => {
                let m = pm.primive_move_to_move(&p);
                g.moves.push(m);
                p.make_move(&m).unwrap();
            }
            Step::IssueOfGame(i) => g.issue = Some(i),
        }
    }
    Ok(g)
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use super::Step;
    use super::PrimitiveMove;
    use types::*;

    #[test]
    fn it_works() {
        assert_eq!(super::parse_move(b"+7776FU,L599"),
                   IResult::Done(&b""[..],
                                 PrimitiveMove::new (
                                     Color::Black,
                                     Some(Point::one_start(7, 7)),
                                     Point::one_start(7, 6),
                                     Piece::Pawn,
                                 )));
        assert_eq!(super::parse_issue_of_game(b"GOTE_WIN_TORYO"),
                   IResult::Done(&b""[..], IssueOfGame::Win(Color::White, Win::Toryo)));
        assert_eq!(super::parse_step(b"	+7776FU,L599	-3334FU,L599	GOTE_WIN_TORYO"),
                   IResult::Done(&b"	-3334FU,L599	GOTE_WIN_TORYO"[..],
                                 Step::Move(PrimitiveMove::new (
                                     Color::Black,
                                     Some(Point::one_start(7, 7)),
                                     Point::one_start(7, 6),
                                     Piece::Pawn,
                                 ))));
        assert_eq!(super::parse_step(b"GOTE_WIN_TORYO"),
                   IResult::Done(&b""[..],
                                 Step::IssueOfGame(IssueOfGame::Win(Color::White, Win::Toryo))));
        assert_eq!(super::parse_kifu_primitive(b"+7776FU,L599	-3334FU,L599	GOTE_WIN_TORYO"),
                   IResult::Done(&b""[..],
                                 vec![Step::Move(PrimitiveMove::new(
                                          Color::Black,
                                          Some(Point::one_start(7, 7)),
                                          Point::one_start(7, 6),
                                          Piece::Pawn,
                                          )),
                                      Step::Move(PrimitiveMove::new (
                                          Color::White,
                                          Some(Point::one_start(3, 3)),
                                          Point::one_start(3, 4),
                                          Piece::Pawn,
                                      )),
                                      Step::IssueOfGame(IssueOfGame::Win(Color::White,
                                                                         Win::Toryo))]));
        let g =super::parse(b"+7776FU,L599	-3334FU,L599	+2726FU,L598	-8384FU,L596	+2625FU,L588	-4132KI,L593	+6978KI,L587	-2288UM,L589	+7988GI,L585	-3122GI,L588	+3938GI,L583	-2233GI,L586	+8877GI,L581	-7172GI,L585	+1716FU,L577	-1314FU,L583	+9796FU,L565	-9394FU,L580	+4746FU,L562	-5142OU,L578	+3847GI,L561	-7283GI,L576	+3736FU,L557	-6152KI,L573	+2937KE,L556	-8485FU,L569	+5968OU,L554	-8384GI,L567	+6879OU,L552	-9495FU,L537	+9695FU,L550	-8495GI,L535	+0094FU,L540	-8586FU,L521	+8786FU,L538	-9586GI,L519	+7786GI,L536	-8286HI,L517	+0087FU,L535	-8683HI,L505	+0072KA,L520	-8385HI,L477	+9493TO,L511	-9193KY,L437	+9993NY,L508	-8193KE,L434	+7294UM,L505	-8555HI,L401	+5756FU,L499	-5554HI,L395	+9493UM,L497	-0098FU,L377	+0055KY,L491	-5464HI,L371	+6766FU,L485	-9899TO,L367	+3745KE,L483	-9989TO,L362	+7989OU,L482	-3344GI,L340	+2524FU,L479	-2324FU,L336	+0065KE,L471	-4445GI,L269	+4645FU,L466	-0095KY,L250	+0061GI,L460	-5262KI,L232	+9371UM,L456	-0085KE,L230	+7162UM,L453	-0057KA,L229	+6253UM,L445	-4233OU,L227	+5364UM,L429	-0098GI,L226	+8988OU,L426	-9899GI,L210	+8889OU,L421	-0077KE,L202	+7877KI,L416	-8577NK,L200	+0042GI,L406	-3323OU,L181	+0033KI,L400	-2133KE,L177	+4233NG,L397	-2333OU,L174	+0025KE,L392	-2425FU,L169	+6442UM,L371	-3242KI,L166	+0031HI,L368	-0032KE,L160	GOTE_WIN_TORYO").unwrap();
        assert_eq!(::encoder::usi::position(&g.position, &g.moves), "position sfen lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 moves 7g7f 3c3d 2g2f 8c8d 2f2e 4a3b 6i7h 2b8h+ 7i8h 3a2b 3i3h 2b3c 8h7g 7a7b 1g1f 1c1d 9g9f 9c9d 4g4f 5a4b 3h4g 7b8c 3g3f 6a5b 2i3g 8d8e 5i6h 8c8d 6h7i 9d9e 9f9e 8d9e P*9d 8e8f 8g8f 9e8f 7g8f 8b8f P*8g 8f8c B*7b 8c8e 9d9c+ 9a9c 9i9c+ 8a9c 7b9d+ 8e5e 5g5f 5e5d 9d9c p*9h L*5e 5d6d 6g6f 9h9i+ 3g4e 9i8i 7i8i 3c4d 2e2d 2c2d N*6e 4d4e 4f4e l*9e S*6a 5b6b 9c7a n*8e 7a6b b*5g 6b5c 4b3c 5c6d s*9h 8i8h 9h9i 8h8i n*7g 7h7g 8e7g+ S*4b 3c2c G*3c 2a3c 4b3c+ 2c3c N*2e 2d2e 6d4b 3b4b R*3a n*3b");
    }
}
