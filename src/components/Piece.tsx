import { motion, useMotionValue } from "framer-motion"
import { MouseEvent, useContext, useEffect, useRef } from "react"

import CursorContext from "../contexts/CursorContext"
import PiecesContext from "../contexts/PiecesContext"
import calculateSquareFromCss from "../functions/calculateSquareFromCss"
import equalSquares from "../functions/equalSquares"
import { Piece as iPiece } from "../types"

const Piece = ({ piece: { id, color, type, square } }: { piece: iPiece }) => {
	const ref = useRef<HTMLDivElement>(null)
	const { isDragging, setIsDragging, setHovered, selected, setSelected } =
		useContext(CursorContext)
	const { pieces, setPieces } = useContext(PiecesContext)
	const x = useMotionValue(0)
	const y = useMotionValue(0)

	useEffect(() => {
		if (ref.current) {
			x.set(0)
			y.set(0)
		}
	}, [ref.current, square])

	const handleMouseDown = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
		setIsDragging(true)
		setSelected({ id, color, type, square })
		setHovered(calculateSquareFromCss(e.currentTarget.style))
	}

	const handleMouseMove = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
		if (isDragging) {
			setHovered(calculateSquareFromCss(e.currentTarget.style))
		}
	}

	const handleMouseUp = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
		if (selected?.id === id) {
			setIsDragging(false)
			setHovered(null)

			// ! Validate before finalizing the move
			const targetSquare = calculateSquareFromCss(e.currentTarget.style)
			const targetPiece = pieces.find(p => equalSquares(p.square, targetSquare))
			if (targetPiece && targetPiece.id !== id) {
				setPieces(
					pieces
						.map(p => (p.id === id ? { ...p, square: targetSquare } : p))
						.filter(p => p.id !== targetPiece.id)
				)
			} else {
				setPieces(pieces.map(p => (p.id === id ? { ...p, square: targetSquare } : p)))
			}
		}
	}

	return (
		<motion.div
			ref={ref}
			style={{
				backgroundImage: `url(./assets/${color}-${type}.png)`,
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
