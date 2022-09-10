import { createContext, Dispatch, PropsWithChildren, SetStateAction, useState } from "react"

import { Square } from "../types"

export enum CursorState {
	Idle,
	Click,
	Drag
}

type iCursorData = {
	cursorState: CursorState
	setCursorState: Dispatch<SetStateAction<CursorState>>
	hovered: Square | null
	setHovered: Dispatch<SetStateAction<Square | null>>
	selected: Square | null
	setSelected: Dispatch<SetStateAction<Square | null>>
}

const context = createContext<iCursorData>({
	cursorState: CursorState.Idle,
	setCursorState: () => {},
	hovered: null,
	setHovered: () => {},
	selected: null,
	setSelected: () => {}
})

export const CursorProvider = ({ children }: PropsWithChildren<{}>) => {
	const [cursorState, setCursorState] = useState(CursorState.Idle)
	const [hovered, setHovered] = useState<Square | null>(null)
	const [selected, setSelected] = useState<Square | null>(null)
	return (
		<context.Provider
			value={{
				cursorState,
				setCursorState,
				hovered,
				setHovered,
				selected,
				setSelected
			}}>
			{children}
		</context.Provider>
	)
}

export default context
