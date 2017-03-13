use json::{self, JsonValue};
use super::super::types::*;
use super::super::parser;

pub fn board(b: &Board) -> JsonValue {
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

pub fn captured(cap: &Captured) -> JsonValue {
    let mut ret = json::object::Object::new();
    for (&(c, p), &n) in cap {
        ret.insert(&format!("{}_{}", c as usize, p as usize),
                   JsonValue::Number(json::number::Number::from(n)));
    }
    JsonValue::Object(ret)
}

pub fn position(p: &Position) -> JsonValue {
    object! {
    "board" => board(p.board()),
    "color" => p.color() as usize,
    "captured" => captured(p.captured())
  }
}

pub fn point(p: &Point) -> JsonValue {
    object! {
      "x" => p.x,
      "y" => p.y
    }
}

pub fn enc_move(m: &Move) -> JsonValue {
    object! {
        "from" => match m.from() {
            Some(p)=> point(&p),
            None => JsonValue::Null
        },
        "to" => point(&m.to()),
        "promote" => JsonValue::Boolean(m.is_promote())
    }
}

pub fn score(s: &parser::usi::Score) -> JsonValue {
    match *s {
        parser::usi::Score::Cp(n) => {
            object! {
                "type" => JsonValue::String("cp".to_string()),
                "value" => JsonValue::Number(n.into())
            }
        }
        parser::usi::Score::Mate(n) => {
            object! {
                "type" => JsonValue::String("mate".to_string()),
                "value" => JsonValue::Number(n.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use types::*;

    #[test]
    fn it_works() {
        println!("{}", super::board(&Board::hirate()).dump());
        assert_eq!(super::board(&Board::hirate()).dump(), r#"[[{"color":1,"piece":1},{"color":1,"piece":2},{"color":1,"piece":3},{"color":1,"piece":4},{"color":1,"piece":7},{"color":1,"piece":4},{"color":1,"piece":3},{"color":1,"piece":2},{"color":1,"piece":1}],[null,{"color":1,"piece":5},null,null,null,null,null,{"color":1,"piece":6},null],[{"color":1,"piece":0},{"color":1,"piece":0},{"color":1,"piece":0},{"color":1,"piece":0},{"color":1,"piece":0},{"color":1,"piece":0},{"color":1,"piece":0},{"color":1,"piece":0},{"color":1,"piece":0}],[null,null,null,null,null,null,null,null,null],[null,null,null,null,null,null,null,null,null],[null,null,null,null,null,null,null,null,null],[{"color":0,"piece":0},{"color":0,"piece":0},{"color":0,"piece":0},{"color":0,"piece":0},{"color":0,"piece":0},{"color":0,"piece":0},{"color":0,"piece":0},{"color":0,"piece":0},{"color":0,"piece":0}],[null,{"color":0,"piece":6},null,null,null,null,null,{"color":0,"piece":5},null],[{"color":0,"piece":1},{"color":0,"piece":2},{"color":0,"piece":3},{"color":0,"piece":4},{"color":0,"piece":7},{"color":0,"piece":4},{"color":0,"piece":3},{"color":0,"piece":2},{"color":0,"piece":1}]]"#);
    }
}
