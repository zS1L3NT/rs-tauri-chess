use crate::{engine::r#move::Move, square, pawn};

#[test]
fn equality() {
    assert_eq!(
        Move::from_normal(square!(E _2), square!(E _3),),
        Move::from_normal(square!(E _2), square!(E _3),)
    );
    assert_ne!(
        Move::from_normal(square!(E _2), square!(E _3),),
        Move::from_normal(square!(E _2), square!(E _4),)
    );
    assert_ne!(
        Move::from_normal(square!(E _2), square!(E _3),),
        Move::from_capture(square!(E _2), square!(E _3), pawn!(0, White))
    );
}
