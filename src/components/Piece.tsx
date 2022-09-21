import { motion, useMotionValue } from "framer-motion"
import { useContext, useEffect, useRef, useState } from "react"

import { invoke } from "@tauri-apps/api"

import CursorContext from "../contexts/CursorContext"
import MovesContext from "../contexts/MovesContext"
import PiecesContext from "../contexts/PiecesContext"
import calculateSquareFromCss from "../functions/calculateSquareFromCss"
import equalSquares from "../functions/equalSquares"
import { Board, Piece as iPiece } from "../types"

const Piece = ({ piece: { id, color, type, square } }: { piece: iPiece }) => {
	const ref = useRef<HTMLDivElement>(null)
	const { isDragging, setIsDragging, setHovered, selected, setSelected } =
		useContext(CursorContext)
	const { moves, setMoves } = useContext(MovesContext)
	const { pieces, setPieces } = useContext(PiecesContext)
	const x = useMotionValue(0)
	const y = useMotionValue(0)

	/**
	 * Tracks whether the current click has intentions of de-selecting the piece
	 *
	 * When null, the piece is not being tracked
	 * When false, the piece didn't move at all so it should be de-selected
	 * When true, the piece moved so it should stay selected
	 */
	const [moved, setMoved] = useState<boolean | null>(null)

	useEffect(() => {
		if (ref.current && !isDragging) {
			x.set(0)
			y.set(0)
		}
	}, [ref.current, isDragging, square])

	const handleMouseDown = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
		if (e.button !== 0) return

		// Piece is already selected, track what happens to it
		if (selected?.id === id) {
			setMoved(false)
		}

		setIsDragging(true)
		setSelected({ id, color, type, square })
		setHovered(calculateSquareFromCss(e.currentTarget.style))
	}

	const handleMouseMove = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
		if (e.button !== 0) return
		if (!isDragging) return

		// If it's moved, it should stay selected
		if (moved !== null) {
			setMoved(true)
		}

		setHovered(calculateSquareFromCss(e.currentTarget.style))
	}

	const handleMouseUp = async (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
		if (e.button !== 0) return
		if (selected?.id !== id) return

		setIsDragging(false)
		setHovered(null)
		setMoved(null)

		// If it didn't move, de-select it
		if (moved === false) {
			setSelected(null)
			return
		}

		const targetSquare = calculateSquareFromCss(e.currentTarget.style)
		const move = moves.find(
			m => equalSquares(m.from, square) && equalSquares(m.to, targetSquare)
		)
		if (move) {
			const board = await invoke<Board>("execute", { move })
			console.log(board)
			setPieces(board.pieces)
			setMoves(board.moves)
		}

		// ! Validate before finalizing the move
		// const targetPiece = pieces.find(p => equalSquares(p.square, targetSquare))
		// if (targetPiece && targetPiece.id !== id) {
		// 	setPieces(
		// 		pieces
		// 			.map(p => (p.id === id ? { ...p, square: targetSquare } : p))
		// 			.filter(p => p.id !== targetPiece.id)
		// 	)
		// } else {
		// 	setPieces(pieces.map(p => (p.id === id ? { ...p, square: targetSquare } : p)))
		// }
	}

	return (
		<motion.div
			ref={ref}
			style={{
				backgroundImage: `url(./assets/${color}${type}.png)`,
				backgroundSize: "cover",
				width: 100,
				height: 100,
				cursor: "grab",
				position: "absolute",
				zIndex: selected?.id === id ? 10 : 1,
				top: 700 - square.rank * 100,
				left: square.file * 100,
				x,
				y
			}}
			drag
			dragConstraints={{
				top: (square.rank - 7) * 100,
				right: (7 - square.file) * 100,
				bottom: square.rank * 100,
				left: -square.file * 100
			}}
			dragMomentum={false}
			dragElastic={0.1}
			onMouseDown={handleMouseDown}
			onMouseMove={handleMouseMove}
			onMouseUp={handleMouseUp}
		/>
	)
}

export default Piece
