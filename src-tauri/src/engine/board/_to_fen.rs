use rs_tauri_chess::square;

use crate::engine::{
    color::Color,
    square::{File, Rank},
};

use super::Board;

impl Board {
    pub fn to_fen(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.generate_piece_placements(),
            self.generate_active_color(),
            self.generate_castling_rights(),
            self.generate_enpassant_square(),
            self.generate_halfmove_clock(),
            self.generate_fullmove_number()
        )
    }

    fn generate_piece_placements(&self) -> String {
        let mut piece_placements = String::new();

        for rank_index in (0..=7).rev() {
            let rank = Rank::from_index(rank_index).unwrap();
            let mut empty_squares = 0;

            for file_index in 0..=7 {
                let file = File::from_index(file_index).unwrap();
                let square = square!(file rank);

                if let Some(piece) = self.pieces.get(&square) {
                    if empty_squares > 0 {
                        piece_placements.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }

                    let mut piece_code = format!("{:?}", piece.r#type);
                    if piece.color == Color::Black {
                        piece_code = piece_code.to_ascii_lowercase();
                    }

                    piece_placements.push_str(&piece_code);
                } else {
                    empty_squares += 1;
                }
            }

            if empty_squares > 0 {
                piece_placements.push_str(&empty_squares.to_string());
            }

            piece_placements.push('/');
        }

        piece_placements.pop();
        piece_placements
    }

    fn generate_active_color(&self) -> String {
        match self.turn {
            Color::White => "w".into(),
            Color::Black => "b".into(),
        }
    }

    fn generate_castling_rights(&self) -> String {
        let mut castling_rights = String::new();

        if self.castling_rights.get(&Color::White).unwrap()[0] {
            castling_rights.push('K');
        }

        if self.castling_rights.get(&Color::White).unwrap()[1] {
            castling_rights.push('Q');
        }

        if self.castling_rights.get(&Color::Black).unwrap()[0] {
            castling_rights.push('k');
        }

        if self.castling_rights.get(&Color::Black).unwrap()[1] {
            castling_rights.push('q');
        }

        if castling_rights.is_empty() {
            castling_rights.push('-');
        }

        castling_rights
    }

    fn generate_enpassant_square(&self) -> String {
        match self.enpassant_square {
            Some(square) => format!("{:?}", square).to_ascii_lowercase(),
            None => "-".into(),
        }
    }

    fn generate_halfmove_clock(&self) -> String {
        self.halfmove_clock.to_string()
    }

    fn generate_fullmove_number(&self) -> String {
        self.fullmove_number.to_string()
    }
}
