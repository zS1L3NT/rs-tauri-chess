use super::{piece::PieceType, square::Square};

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Normal,
    Capture,
    Promotion,
    PromotionCapture,
    PawnJump,
    Enpassant,
    Castle,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub r#type: MoveType,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn from_normal(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Normal,
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
            promotion: None,
        }
    }

    pub fn from_capture(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Capture,
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
            promotion: Some(promotion),
        }
    }

    pub fn from_promotion_capture(
        from: Square,
        to: Square,
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
            promotion: Some(promotion),
        }
    }

    pub fn from_enpassant(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        assert_eq!((from.file as i8 - to.file as i8).abs(), 1);
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 1);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Enpassant,
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
            promotion: None,
        }
    }
}
