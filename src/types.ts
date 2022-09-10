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

export enum Type {
	King = "king",
	Queen = "queen",
	Rook = "rook",
	Bishop = "bishop",
	Knight = "knight",
	Pawn = "pawn"
}

export enum Color {
	White = "white",
	Black = "black"
}

export interface Piece {
	id: number
	type: Type
	color: Color
	square: Square
}
