use super::square::Square;

#[derive(Debug, PartialEq)]
pub struct AttackLines {
    /// The square of the attacker
    pub origin: Square,
    /// The squares attacked in this line
    ///
    /// The order of the vector starts from the attacker and ends at the end of the board / move
    pub lines: Vec<Vec<Square>>,
    /// Index of the line that lines up with the enemy King
    ///
    /// Doesn't matter if there are multiple pieces between the attacker and the King
    pub lines_with_king: Option<usize>,
}

impl AttackLines {
    pub fn new(
        origin: Square,
        lines: Vec<Vec<Square>>,
        lines_with_king: Option<usize>,
    ) -> AttackLines {
        AttackLines {
            origin,
            lines,
            lines_with_king,
        }
    }
}
