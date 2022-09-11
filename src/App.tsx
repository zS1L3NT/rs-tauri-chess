import { useContext } from "react"

import Board from "./components/Board"
import MoveHover from "./components/MoveHover"
import Piece from "./components/Piece"
import PiecesContext from "./contexts/PiecesContext"

const App = () => {
	const { pieces } = useContext(PiecesContext)

	return (
		<div
			style={{
				position: "relative",
				overflow: "hidden"
			}}>
			<Board />
			<MoveHover />
			{pieces.map(piece => (
				<Piece
					key={piece.id}
					piece={piece}
				/>
			))}
		</div>
	)
}

export default App
