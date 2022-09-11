use crate::square;

#[test]
fn equality() {
    assert_eq!(square!(E _2), square!(E _2));
    assert_ne!(square!(E _2), square!(E _3));
    assert_ne!(square!(E _2), square!(F _2));
}
