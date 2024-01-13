import { useEffect, useState } from "react"

import { invoke } from "@tauri-apps/api"

import { setBoard } from "../slices/BoardSlice"
import { Board } from "../types"
import useAppDispatch from "./useAppDispatch"

const useListenReset = () => {
	const dispatch = useAppDispatch()

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
				dispatch(setBoard(await invoke<Board>("reset")))
			}
		}
	}
}

export default useListenReset
