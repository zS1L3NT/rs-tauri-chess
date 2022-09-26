import { invoke } from "@tauri-apps/api"

import equalSquares from "../functions/equalSquares"
import useAppDispatch from "../hooks/useAppDispatch"
import useAppSelector from "../hooks/useAppSelector"
import { setBoard } from "../slices/BoardSlice"
import { setPromotion } from "../slices/PromotionSlice"
import { Board, Square } from "../types"

const Target = ({
	square,
	isCapture,
	isPromotion
}: {
	square: Square
	isCapture: boolean
	isPromotion: boolean
}) => {
	const dispatch = useAppDispatch()
	const moves = useAppSelector(state => state.board.moves)
	const selected = useAppSelector(state => state.cursor.selected)

	const handleClick = async () => {
		if (isPromotion) {
			dispatch(setPromotion(square))
		} else {
			const move = moves.find(
				m => equalSquares(m.from, selected!.square) && equalSquares(m.to, square)
			)!
			dispatch(setBoard(await invoke<Board>("execute", { move })))
		}
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
