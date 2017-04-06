pub mod usi;
pub mod csa;
pub mod japanese;

use super::types::*;

#[derive(PartialEq, Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Movement {
    pub movement: Option<Move>,
    pub movestr: Option<String>,
    pub position: Position,
}

pub fn get_moves(g: &Game) -> Result<Vec<Movement>, String> {
    let mut p = Position::hirate();
    let mut kifu = Vec::new();
    kifu.push(Movement {
        movement: None,
        movestr: None,
        position: p.clone(),
    });
    for m in g.moves.iter() {
        p.make_move(m)?;
        kifu.push(Movement {
            movement: Some(m.clone()),
            movestr: Some(japanese::enc_move(m)),
            position: p.clone(),
        });
    }

    Ok(kifu)
}
