#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CastlingRights {
    pub kingside: bool,
    pub queenside: bool,
}

impl CastlingRights {
    pub fn new(kingside: bool, queenside: bool) -> Self {
        Self {
            kingside,
            queenside,
        }
    }
}
