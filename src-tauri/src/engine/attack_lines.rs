use super::square::Square;

#[derive(PartialEq)]
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

impl std::fmt::Debug for AttackLines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AttackLines({:#?})",
            self.lines
                .iter()
                .enumerate()
                .map(|(i, line)| format!(
                    "{}{:?}->{}{}",
                    match self.lines_with_king {
                        Some(index) if index == i => "*",
                        _ => "",
                    },
                    self.origin,
                    line.iter()
                        .map(|s| format!("{:?}", s))
                        .collect::<Vec<_>>()
                        .join("->"),
                    match self.lines_with_king {
                        Some(index) if index == i => "*",
                        _ => "",
                    },
                ))
                .collect::<Vec<_>>()
        )
    }
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
