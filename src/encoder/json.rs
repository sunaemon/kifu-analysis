use json::JsonValue;
use super::super::types::*;

pub fn board(b: Board) -> JsonValue {
    let mut ret = Vec::new();
    for y in 1..10 {
        let mut row = Vec::new();
        for x in 1..10 {
            match b[Point::one_start(x, y)] {
                Some((c, p)) => {
                    row.push(object! {
                      "color" => c as usize,
                      "piece" => p as usize
                    })
                }
                None => row.push(JsonValue::Null),
            }
        }
        ret.push(JsonValue::Array(row));
    }
    JsonValue::Array(ret)
}

#[cfg(test)]
mod tests {
    use types::*;

    #[test]
    fn it_works() {
        assert_eq!(super::board(Board::hirate()).dump(), "");
    }
}
