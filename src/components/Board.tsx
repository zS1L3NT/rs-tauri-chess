import useAppDispatch from "../hooks/useAppDispatch"
import { setCursor } from "../slices/CursorSlice"
import { setPromotion } from "../slices/PromotionSlice"

const Board = () => {
	const dispatch = useAppDispatch()

	const handleClick = () => {
		dispatch(setCursor({ selected: null }))
		dispatch(setPromotion(null))
	}

	return (
		<div
			style={{
				backgroundImage: "url(./assets/Board.png)",
				backgroundSize: "contain",
				width: "800px",
				height: "800px"
			}}
			onClick={handleClick}
		/>
	)
}

export default Board
