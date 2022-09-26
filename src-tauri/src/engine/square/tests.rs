use rs_tauri_chess::square;

#[test]
fn equality() {
    assert_eq!(square!(E2), square!(E2));
    assert_ne!(square!(E2), square!(E3));
    assert_ne!(square!(E2), square!(F2));
}
