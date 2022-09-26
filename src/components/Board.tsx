import { useContext } from "react"

import CursorContext from "../contexts/CursorContext"
import PromotionContext from "../contexts/PromotionContext"

const Board = () => {
	const { setSelected } = useContext(CursorContext)
	const promotion = useContext(PromotionContext)

	const handleClick = () => {
		setSelected(null)
		promotion.setFile(null)
		promotion.setColor(null)
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
