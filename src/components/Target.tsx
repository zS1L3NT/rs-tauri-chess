import { useContext } from "react"

import { invoke } from "@tauri-apps/api"

import CursorContext from "../contexts/CursorContext"
import MovesContext from "../contexts/MovesContext"
import PiecesContext from "../contexts/PiecesContext"
import equalSquares from "../functions/equalSquares"
import { Board, Square } from "../types"

const Target = ({ square, isCapture }: { square: Square; isCapture: boolean }) => {
	const { selected } = useContext(CursorContext)
	const { moves, setMoves } = useContext(MovesContext)
	const { setPieces } = useContext(PiecesContext)

	const handleClick = async (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
		const move = moves.find(
			m => equalSquares(m.from, selected!.square) && equalSquares(m.to, square)
		)!

		const board = await invoke<Board>("execute", { move })
		console.log(board)
		setPieces(board.pieces)
		setMoves(board.moves)
	}

	return (
		<div
			style={{
				width: 100,
				height: 100,
				cursor: "pointer",
				position: "absolute",
				zIndex: 3,
				top: 700 - square.rank * 100,
				left: square.file * 100
			}}
			onClick={handleClick}>
			{isCapture ? (
				<div
					style={{
						width: 100,
						height: 100,
						border: "10px solid rgba(0, 0, 0, 0.1)",
						borderRadius: "50%"
					}}
				/>
			) : (
				<div
					style={{
						width: "36px",
						height: "36px",
						margin: "32px",
						background: "rgba(0, 0, 0, 0.1)",
						borderRadius: "50%"
					}}
				/>
			)}
		</div>
	)
}

export default Target
