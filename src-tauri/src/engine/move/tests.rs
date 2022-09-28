use {
    crate::{engine::r#move::*, pawn},
    rs_tauri_chess::square,
};

#[test]
fn equality() {
    assert_eq!(
        Move::from_normal(square!(E2), square!(E3),),
        Move::from_normal(square!(E2), square!(E3),)
    );
    assert_ne!(
        Move::from_normal(square!(E2), square!(E3),),
        Move::from_normal(square!(E2), square!(E4),)
    );
    assert_ne!(
        Move::from_normal(square!(E2), square!(E3),),
        Move::from_capture(square!(E2), square!(E3), pawn!(0, White))
    );
}
