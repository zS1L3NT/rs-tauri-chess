import { useContext } from "react"

import CursorContext from "../contexts/CursorContext"

const Board = () => {
	const { setSelected } = useContext(CursorContext)

	return (
		<div
			style={{
				backgroundImage: "url(./assets/Board.png)",
				backgroundSize: "contain",
				width: "800px",
				height: "800px"
			}}
			onClick={() => setSelected(null)}
		/>
	)
}

export default Board
