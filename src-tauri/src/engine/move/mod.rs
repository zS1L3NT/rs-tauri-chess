mod move_type;
#[cfg(test)]
mod tests;

pub use move_type::MoveType;

use serde::{Deserialize, Serialize};

use super::{
    piece::{Piece, PieceType},
    square::Square,
};

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub r#type: MoveType,
    pub captured: Option<Piece>,
    pub promotion: Option<PieceType>,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}Move({:?}->{:?}{}{})",
            self.r#type,
            self.from,
            self.to,
            match &self.captured {
                Some(captured) => format!(" ~{:?}", captured),
                None => "".into(),
            },
            match &self.promotion {
                Some(promotion) => format!(" ^{:?}", promotion),
                None => "".into(),
            }
        )
    }
}

impl Move {
    pub fn from_normal(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Normal,
            captured: None,
            promotion: None,
        }
    }

    pub fn from_pawn_jump(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        assert_eq!(from.file, to.file);
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 2);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::PawnJump,
            captured: None,
            promotion: None,
        }
    }

    pub fn from_capture(from: Square, to: Square, captured: Piece) -> Move {
        assert_ne!(from, to);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Capture,
            captured: Some(captured),
            promotion: None,
        }
    }

    pub fn from_promotion(from: Square, to: Square, promotion: PieceType) -> Move {
        assert_ne!(from, to);
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 1);
        assert_ne!(promotion, PieceType::Pawn);
        assert_ne!(promotion, PieceType::King);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Promotion,
            captured: None,
            promotion: Some(promotion),
        }
    }

    pub fn from_promotion_capture(
        from: Square,
        to: Square,
        captured: Piece,
        promotion: PieceType,
    ) -> Move {
        assert_ne!(from, to);
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 1);
        assert_ne!(promotion, PieceType::Pawn);
        assert_ne!(promotion, PieceType::King);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::PromotionCapture,
            captured: Some(captured),
            promotion: Some(promotion),
        }
    }

    pub fn from_enpassant(from: Square, to: Square, captured: Piece) -> Move {
        assert_ne!(from, to);
        assert_eq!((from.file as i8 - to.file as i8).abs(), 1);
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 1);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Enpassant,
            captured: Some(captured),
            promotion: None,
        }
    }

    pub fn from_castle(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        assert_eq!((from.file as i8 - to.file as i8).abs(), 2);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Castle,
            captured: None,
            promotion: None,
        }
    }
}