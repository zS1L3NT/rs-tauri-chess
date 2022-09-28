use {
    crate::engine::{client::*, r#move::*},
    serde::Serialize,
};

#[derive(Clone, Serialize)]
pub struct ClientBoard {
    pieces: Vec<ClientPiece>,
    moves: Vec<Move>,
}

impl ClientBoard {
    pub fn new(pieces: Vec<ClientPiece>, moves: Vec<Move>) -> ClientBoard {
        ClientBoard { pieces, moves }
    }
}
