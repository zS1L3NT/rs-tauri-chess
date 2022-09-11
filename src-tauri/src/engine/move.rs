use super::{piece::PieceType, square::Square};

pub enum MoveType {
    Normal,
    Capture,
    Promotion,
    PromotionCapture,
    PawnJump,
    Enpassant,
    Castle,
}

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub r#type: MoveType,
    pub promotion: Option<PieceType>,
    pub threat: bool,
}

impl Move {
    pub fn fromNormal(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Normal,
            promotion: None,
            threat: false,
        }
    }

    pub fn fromPawnJump(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        assert_eq!(from.file, to.file);
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 2);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::PawnJump,
            promotion: None,
            threat: false,
        }
    }

    pub fn fromCapture(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Capture,
            promotion: None,
            threat: false,
        }
    }

    pub fn fromPromotion(from: Square, to: Square, promotion: PieceType, threat: bool) -> Move {
        assert_ne!(from, to);
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 1);
        assert_ne!(promotion, PieceType::Pawn);
        assert_ne!(promotion, PieceType::King);
        assert_ne!(threat, promotion == PieceType::Knight);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Promotion,
            promotion: Some(promotion),
            threat,
        }
    }

    pub fn fromPromotionCapture(
        from: Square,
        to: Square,
        promotion: PieceType,
        threat: bool,
    ) -> Move {
        assert_ne!(from, to);
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 1);
        assert_ne!(promotion, PieceType::Pawn);
        assert_ne!(promotion, PieceType::King);
        assert_ne!(threat, promotion == PieceType::Knight);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::PromotionCapture,
            promotion: Some(promotion),
            threat,
        }
    }

    pub fn fromEnpassent(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        assert_eq!((from.file as i8 - to.file as i8).abs(), 1);
        assert_eq!((from.rank as i8 - to.rank as i8).abs(), 1);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Enpassant,
            promotion: None,
            threat: false,
        }
    }

    pub fn fromCastle(from: Square, to: Square) -> Move {
        assert_ne!(from, to);
        assert_eq!((from.file as i8 - to.file as i8).abs(), 2);
        Move {
            from: from.clone(),
            to,
            r#type: MoveType::Castle,
            promotion: None,
            threat: false,
        }
    }
}