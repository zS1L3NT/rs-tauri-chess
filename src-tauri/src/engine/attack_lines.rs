use super::square::Square;

pub struct AttackLines {
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
    pub fn new(lines: Vec<Vec<Square>>, lines_with_king: Option<usize>) -> AttackLines {
        AttackLines {
            lines,
            lines_with_king,
        }
    }
}
