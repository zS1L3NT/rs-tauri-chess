export enum File {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H
}

export enum Rank {
	_1,
	_2,
	_3,
	_4,
	_5,
	_6,
	_7,
	_8
}

export interface Square {
	file: File
	rank: Rank
}

export enum Color {
	White = "White",
	Black = "Black"
}

export enum PieceType {
	King = "King",
	Queen = "Queen",
	Rook = "Rook",
	Bishop = "Bishop",
	Knight = "Knight",
	Pawn = "Pawn"
}

export interface Piece {
	id: number
	type: PieceType
	color: Color
	square: Square
}

export enum MoveType {
	Normal = "Normal",
	Capture = "Capture",
	Promotion = "Promotion",
	PromotionCapture = "PromotionCapture",
	PawnJump = "PawnJump",
	Enpassant = "Enpassant",
	Castle = "Castle"
}

export interface Move {
	from: Square
	to: Square
	type: MoveType
	captured: Piece | null
	promotion: PieceType | null
}
