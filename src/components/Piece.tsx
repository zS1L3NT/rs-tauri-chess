import { motion } from "framer-motion"
import { MouseEvent, useContext } from "react"

import CursorContext, { CursorState } from "../contexts/CursorContext"
import { Piece as iPiece } from "../types"

const Piece = ({ piece: { color, type, square } }: { piece: iPiece }) => {
	const { cursorState, setCursorState, selected, setSelected } = useContext(CursorContext)

	const isSelected = square.file === selected?.file && square.rank === selected?.rank

	const handleMouseDown = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
		setCursorState(CursorState.Click)
	}

	const handleMouseMove = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
		setCursorState(CursorState.Drag)
	}

	const handleMouseUp = (e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>) => {
		setCursorState(CursorState.Idle)
		if (cursorState === CursorState.Click) {
			setSelected(isSelected ? null : square)
		}
	}

	return (
		<motion.div
			style={{
				backgroundImage: `url(./assets/${color}-${type}.png)`,
				backgroundSize: "cover",
				width: 100,
				height: 100,
				cursor: "grab",
				position: "absolute",
				zIndex: 1,
				top: 700 - square.rank * 100,
				left: square.file * 100
			}}
			animate={{
				backgroundColor: isSelected ? "rgba(245, 204, 42, .5)" : "rgba(245, 204, 42, 0)",
				top: 700 - square.rank * 100,
				left: square.file * 100
			}}
			transition={{ duration: 0.25 }}
			drag
			dragConstraints={{
				top: (square.rank - 7) * 100,
				right: (7 - square.file) * 100,
				bottom: square.rank * 100,
				left: -square.file * 100
			}}
			dragMomentum={false}
			onMouseDown={handleMouseDown}
			onMouseMove={handleMouseMove}
			onMouseUp={handleMouseUp}
		/>
	)
}

export default Piece
