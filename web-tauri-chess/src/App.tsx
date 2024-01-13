import useAsyncEffect from "use-async-effect"

import { invoke } from "@tauri-apps/api"

import Board from "./components/Board"
import Highlight from "./components/Highlight"
import Hover from "./components/Hover"
import Piece from "./components/Piece"
import Promotion from "./components/Promotion"
import Target from "./components/Target"
import equalSquares from "./functions/equalSquares"
import useAppDispatch from "./hooks/useAppDispatch"
import useAppSelector from "./hooks/useAppSelector"
import useListenReset from "./hooks/useListenReset"
import { setBoard } from "./slices/BoardSlice"
import { Board as iBoard, MoveType, PieceType } from "./types"

const App = () => {
	const dispatch = useAppDispatch()
	const { moves, pieces } = useAppSelector(state => state.board)
	const promotion = useAppSelector(state => state.promotion)
	const selected = useAppSelector(state => state.cursor.selected)

	useListenReset()

	useAsyncEffect(async () => {
		// @ts-ignore
		window.invoke = invoke

		dispatch(setBoard(await invoke<iBoard>("state")))
	}, [])

	return (
		<div
			style={{
				position: "relative",
				overflow: "hidden",
			}}>
			<Board />
			<Hover />
			{promotion ? <Promotion /> : null}
			{selected ? <Highlight square={selected.square} /> : null}
			{selected
				? moves
						.filter(m => equalSquares(m.from, selected.square))
						.map(m =>
							!m.promotion || m.promotion === PieceType.Queen ? (
								<Target
									key={`${m.to.file}${m.to.rank}${m.promotion}`}
									square={m.to}
									isCapture={
										m.type === MoveType.Capture ||
										m.type === MoveType.PromotionCapture ||
										m.type === MoveType.Enpassant
									}
									isPromotion={!!m.promotion}
								/>
							) : null,
						)
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
