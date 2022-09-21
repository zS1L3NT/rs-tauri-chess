import { useContext } from "react"
import useAsyncEffect from "use-async-effect"

import { invoke } from "@tauri-apps/api"

import Board from "./components/Board"
import Highlight from "./components/Highlight"
import Hover from "./components/Hover"
import Piece from "./components/Piece"
import Target from "./components/Target"
import CursorContext from "./contexts/CursorContext"
import MovesContext from "./contexts/MovesContext"
import PiecesContext from "./contexts/PiecesContext"
import equalSquares from "./functions/equalSquares"
import { Board as iBoard, MoveType } from "./types"

const App = () => {
	const { selected } = useContext(CursorContext)
	const { moves, setMoves } = useContext(MovesContext)
	const { pieces, setPieces } = useContext(PiecesContext)

	useAsyncEffect(async () => {
		const board = await invoke<iBoard>("state")
		setPieces(board.pieces)
		setMoves(board.moves)
	}, [])

	return (
		<div
			style={{
				position: "relative",
				overflow: "hidden"
			}}>
			<Board />
			<Hover />
			{selected ? <Highlight square={selected.square} /> : null}
			{selected
				? moves
						.filter(m => equalSquares(m.from, selected.square))
						.map(m => (
							<Target
								key={`${m.to.file}${m.to.rank}`}
								square={m.to}
								isCapture={
									m.type === MoveType.Capture ||
									m.type === MoveType.PromotionCapture
								}
							/>
						))
				: null}
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
