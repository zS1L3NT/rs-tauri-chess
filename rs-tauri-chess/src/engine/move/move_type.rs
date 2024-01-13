pub use MoveType::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Deserialize, PartialEq, Serialize)]
pub enum MoveType {
    Normal,
    Capture,
    Promotion,
    PromotionCapture,
    PawnJump,
    Enpassant,
    Castle,
}
