import { useContext, useState } from "react"
import useAsyncEffect from "use-async-effect"

import { invoke } from "@tauri-apps/api"

import Board from "./components/Board"
import Highlight from "./components/Highlight"
import Hover from "./components/Hover"
import Piece from "./components/Piece"
import Target from "./components/Target"
import CursorContext from "./contexts/CursorContext"
import PiecesContext from "./contexts/PiecesContext"
import equalSquares from "./functions/equalSquares"
import { Move, MoveType, Piece as iPiece } from "./types"

const App = () => {
	const { selected } = useContext(CursorContext)
	const { pieces, setPieces } = useContext(PiecesContext)

	const [moves, setMoves] = useState<Move[]>([])

	useAsyncEffect(async () => {
		const { pieces, moves } = await invoke<{ pieces: iPiece[]; moves: Move[] }>("state")

		setPieces(pieces)
		setMoves(moves)
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
