import { motion, useMotionValue } from "framer-motion"
import { MouseEvent, useContext, useEffect, useRef } from "react"

import CursorContext from "../contexts/CursorContext"
import PiecesContext from "../contexts/PiecesContext"
import calculateSquareFromCss from "../functions/calculateSquareFromCss"
import { Piece as iPiece } from "../types"

const Piece = ({ piece: { id, color, type, square } }: { piece: iPiece }) => {
	const ref = useRef<HTMLDivElement>(null)
	const { isDragging, setIsDragging, setHovered, selected, setSelected } =
		useContext(CursorContext)
	const { setPieces } = useContext(PiecesContext)
	const x = useMotionValue(0)
	const y = useMotionValue(0)

	const isSelected = square.file === selected?.file && square.rank === selected?.rank

	useEffect(() => {
		if (ref.current) {
			x.set(0)
			y.set(0)
		}
	}, [ref.current, square])

	const handleMouseDown = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
		setIsDragging(true)
		setSelected(isSelected ? null : square)
		setHovered(calculateSquareFromCss(e.currentTarget.style))
	}

	const handleMouseMove = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
		if (isDragging) {
			setHovered(calculateSquareFromCss(e.currentTarget.style))
		}
	}

	const handleMouseUp = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
		setIsDragging(false)
		setHovered(null)

		// ! Validate before finalizing the move
		setPieces(pieces =>
			pieces.map(p =>
				p.id === id ? { ...p, square: calculateSquareFromCss(e.currentTarget.style) } : p
			)
		)
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
				zIndex: 1,
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
