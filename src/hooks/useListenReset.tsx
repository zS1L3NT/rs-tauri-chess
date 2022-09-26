import { useContext, useEffect, useState } from "react"

import { invoke } from "@tauri-apps/api"

import MovesContext from "../contexts/MovesContext"
import PiecesContext from "../contexts/PiecesContext"
import { Board } from "../types"

const useListenReset = () => {
	const { setMoves } = useContext(MovesContext)
	const { setPieces } = useContext(PiecesContext)

	const [tabPressedAt, setTabPressedAt] = useState<number | null>(null)

	useEffect(() => {
		document.addEventListener("keydown", handleKeyDown)
		return () => document.removeEventListener("keydown", handleKeyDown)
	}, [tabPressedAt])

	const handleKeyDown = async (e: KeyboardEvent) => {
		if (e.key === "Tab") {
			setTabPressedAt(Date.now())
		}

		if (e.key === "Enter") {
			if (tabPressedAt && Date.now() - tabPressedAt < 1000) {
				const board = await invoke<Board>("reset")
				setPieces(board.pieces)
				setMoves(board.moves)
			}
		}
	}
}

export default useListenReset
