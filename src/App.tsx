import { useContext } from "react"

import Board from "./components/Board"
import Highlight from "./components/Highlight"
import MoveHover from "./components/MoveHover"
import Piece from "./components/Piece"
import CursorContext from "./contexts/CursorContext"
import PiecesContext from "./contexts/PiecesContext"

const App = () => {
	const { selected } = useContext(CursorContext)
	const { pieces } = useContext(PiecesContext)

	return (
		<div
			style={{
				position: "relative",
				overflow: "hidden"
			}}>
			<Board />
			<MoveHover />
			{selected ? <Highlight square={selected.square} /> : null}
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
