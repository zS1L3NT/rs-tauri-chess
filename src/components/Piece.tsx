import { motion, useMotionValue } from "framer-motion"
import { useEffect, useRef, useState } from "react"

import { invoke } from "@tauri-apps/api"

import calculateSquareFromCss from "../functions/calculateSquareFromCss"
import equalSquares from "../functions/equalSquares"
import useAppDispatch from "../hooks/useAppDispatch"
import useAppSelector from "../hooks/useAppSelector"
import { setBoard } from "../slices/BoardSlice"
import { setCursor } from "../slices/CursorSlice"
import { setPromotion } from "../slices/PromotionSlice"
import { Board, MoveType, Piece as iPiece } from "../types"

const Piece = ({ piece: { id, color, type, square } }: { piece: iPiece }) => {
	const dispatch = useAppDispatch()
	const { isDragging, selected } = useAppSelector(state => state.cursor)
	const moves = useAppSelector(state => state.board.moves)

	const ref = useRef<HTMLDivElement>(null)
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

		dispatch(
			setCursor({
				isDragging: true,
				selected: { id, color, type, square },
				hovered: calculateSquareFromCss(e.currentTarget.style)
			})
		)
	}

	const handleMouseMove = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
		if (e.button !== 0) return
		if (!isDragging) return

		// If it's moved, it should stay selected
		if (moved !== null) {
			setMoved(true)
		}

		dispatch(
			setCursor({
				hovered: calculateSquareFromCss(e.currentTarget.style)
			})
		)
	}

	const handleMouseUp = async (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
		if (e.button !== 0) return
		if (selected?.id !== id) return

		setMoved(null)
		dispatch(
			setCursor({
				isDragging: false,
				hovered: null
			})
		)

		// If it didn't move, de-select it
		if (moved === false) {
			return dispatch(
				setCursor({
					selected: null
				})
			)
		}

		const targetSquare = calculateSquareFromCss(e.currentTarget.style)
		const move = moves.find(
			m => equalSquares(m.from, square) && equalSquares(m.to, targetSquare)
		)
		if (move) {
			if (move.type === MoveType.Promotion) {
				dispatch(setPromotion(targetSquare))
			} else {
				dispatch(setBoard(await invoke<Board>("execute", { move })))
			}
		}
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
