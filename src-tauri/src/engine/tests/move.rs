use crate::{engine::r#move::Move, square};

#[test]
fn equality() {
    assert_eq!(
        Move::fromNormal(square!(E _2), square!(E _3),),
        Move::fromNormal(square!(E _2), square!(E _3),)
    );
    assert_ne!(
        Move::fromNormal(square!(E _2), square!(E _3),),
        Move::fromNormal(square!(E _2), square!(E _4),)
    );
    assert_ne!(
        Move::fromNormal(square!(E _2), square!(E _3),),
        Move::fromCapture(square!(E _2), square!(E _3),)
    );
}
