use crate::engine::*;

impl Board {
	#[allow(dead_code)]
    pub fn perft(&mut self, depth: usize, log_depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }

        let mut count = 0;

        for r#move in self.get_moves() {
            let castling_rights = self.castling_rights.clone();
            let enpassant_square = self.enpassant_square;
            let halfmove_clock = self.halfmove_clock;

            let move_name = format!("{:?}{:?}", r#move.from, r#move.to).to_lowercase();
            self.execute(r#move);

            let perft = self.perft(depth - 1, log_depth);
            if depth == log_depth {
                println!("{}: {}", move_name, perft);
            }
            count += perft;

            self.undo(castling_rights, enpassant_square, halfmove_clock);
        }

        count
    }
}
