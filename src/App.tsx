import { useContext } from "react"
import useAsyncEffect from "use-async-effect"

import { invoke } from "@tauri-apps/api"

import Board from "./components/Board"
import Highlight from "./components/Highlight"
import Hover from "./components/Hover"
import Piece from "./components/Piece"
import CursorContext from "./contexts/CursorContext"
import PiecesContext from "./contexts/PiecesContext"

const App = () => {
	const { selected } = useContext(CursorContext)
	const { pieces } = useContext(PiecesContext)

	useAsyncEffect(async () => {
		console.log(await invoke("state"))
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
