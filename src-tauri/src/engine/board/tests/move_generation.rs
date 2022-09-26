use super::super::Board;

#[test]
fn normal_d1() {
    assert_eq!(Board::new().count_moves(1), 20);
}

#[test]
fn normal_d2() {
    assert_eq!(Board::new().count_moves(2), 400);
}

#[test]
fn normal_d3() {
    assert_eq!(Board::new().count_moves(3), 8902);
}

#[test]
fn normal_d4() {
    assert_eq!(Board::new().count_moves(4), 197281);
}

#[test]
fn normal_d5() {
    assert_eq!(Board::new().count_moves(5), 4865609);
}

#[test]
fn normal_d6() {
    assert_eq!(Board::new().count_moves(6), 119060324);
}
