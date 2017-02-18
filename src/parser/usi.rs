use types::*;

named!(digit_as_u8<&[u8], u8>,
       map!(one_of!("0123456789"), |d:char| d.to_digit(10).unwrap() as u8));

named!(piece_without_promotion_with_color<&[u8], (Color, Piece)>,
       alt!(
         tag!("P") => { |_| (Color::Black, Piece::Pawn) } |
         tag!("L") => { |_| (Color::Black, Piece::Lance) } |
         tag!("K") => { |_| (Color::Black, Piece::Knight) } |
         tag!("S") => { |_| (Color::Black, Piece::Silver) } |
         tag!("G") => { |_| (Color::Black, Piece::Gold) } |
         tag!("B") => { |_| (Color::Black, Piece::Bishop) } |
         tag!("R") => { |_| (Color::Black, Piece::Rook) } |
         tag!("K") => { |_| (Color::Black, Piece::King) } |
         tag!("p") => { |_| (Color::White, Piece::Pawn) } |
         tag!("l") => { |_| (Color::White, Piece::Lance) } |
         tag!("k") => { |_| (Color::White, Piece::Knight) } |
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
           (c,p) => (c, if promoted.is_some() {p.promote()} else {p})
         }
         ));

named!(point<&[u8], Point>,
       chain!(
         x: digit_as_u8 ~
         y: digit_as_u8,
         || Point::new(x, y)));

#[cfg(test)]
mod tests {
    use nom::IResult;

    use types::*;

    #[test]
    fn it_works() {
        assert_eq!(super::piece_with_color(b"+b"),
                   IResult::Done(&b""[..], (Color::White, Piece::Horse)));
    }
}
