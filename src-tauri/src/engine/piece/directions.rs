pub struct Directions {}

impl Directions {
    pub const KNIGHT: [(i8, i8); 8] = [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];

    pub const BISHOP: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

    pub const ROOK: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub const QUEEN: [(i8, i8); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    pub const KING: [(i8, i8); 8] = Directions::QUEEN;
}