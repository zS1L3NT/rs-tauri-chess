import { useContext } from "react"

import { invoke } from "@tauri-apps/api"

import MovesContext from "../contexts/MovesContext"
import PiecesContext from "../contexts/PiecesContext"
import PromotionContext from "../contexts/PromotionContext"
import { Board, Color, File, PieceType } from "../types"

const Promotion = ({ color, file }: { color: Color; file: File }) => {
	const { moves, setMoves } = useContext(MovesContext)
	const { setPieces } = useContext(PiecesContext)
	const promotion = useContext(PromotionContext)

	const handleClick = async (type: PieceType) => {
		const move = moves.find(m => m.promotion === type && m.to.file === file)!
		const board = await invoke<Board>("execute", { move })
		setPieces(board.pieces)
		setMoves(board.moves)
		promotion.setFile(null)
		promotion.setColor(null)
	}

	return (
		<div
			style={{
				width: 100,
				height: 450,
				position: "absolute",
				top: color == Color.White ? 0 : "unset",
				bottom: color == Color.Black ? 0 : "unset",
				display: "flex",
				flexDirection: color == Color.White ? "column" : "column-reverse",
				left: file * 100,
				zIndex: 11,
				backgroundColor: "white",
				borderRadius: 3,
				boxShadow: "3px 3px 10px rgb(0 0 0 / 45%)"
			}}>
			<div
				style={{
					backgroundImage: `url(./assets/${color}Queen.png)`,
					backgroundSize: "cover",
					width: 100,
					height: 100,
					cursor: "pointer",
					zIndex: 5
				}}
				onClick={() => handleClick(PieceType.Queen)}
			/>
			<div
				style={{
					backgroundImage: `url(./assets/${color}Knight.png)`,
					backgroundSize: "cover",
					width: 100,
					height: 100,
					cursor: "pointer",
					zIndex: 5
				}}
				onClick={() => handleClick(PieceType.Knight)}
			/>
			<div
				style={{
					backgroundImage: `url(./assets/${color}Rook.png)`,
					backgroundSize: "cover",
					width: 100,
					height: 100,
					cursor: "pointer",
					zIndex: 5
				}}
				onClick={() => handleClick(PieceType.Rook)}
			/>
			<div
				style={{
					backgroundImage: `url(./assets/${color}Bishop.png)`,
					backgroundSize: "cover",
					width: 100,
					height: 100,
					cursor: "pointer",
					zIndex: 5
				}}
				onClick={() => handleClick(PieceType.Bishop)}
			/>
			<div
				style={{
					width: 100,
					height: 50,
					backgroundColor: "#F1F1F1",
					borderRadius: color == Color.White ? "0 0 3px 3px" : "3px 3px 0 0",
					display: "flex",
					justifyContent: "center",
					alignItems: "center",
					cursor: "pointer",
					fontWeight: "900",
					fontSize: 24
				}}
				onClick={() => {
					promotion.setFile(null)
					promotion.setColor(null)
				}}>
				&times;
			</div>
		</div>
	)
}

export default Promotion
