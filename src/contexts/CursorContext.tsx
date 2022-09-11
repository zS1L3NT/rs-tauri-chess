import { createContext, Dispatch, PropsWithChildren, SetStateAction, useState } from "react"

import { Piece, Square } from "../types"

type iCursorData = {
	isDragging: boolean
	setIsDragging: Dispatch<SetStateAction<boolean>>
	hovered: Square | null
	setHovered: Dispatch<SetStateAction<Square | null>>
	selected: Piece | null
	setSelected: Dispatch<SetStateAction<Piece | null>>
}

const context = createContext<iCursorData>({
	isDragging: false,
	setIsDragging: () => {},
	hovered: null,
	setHovered: () => {},
	selected: null,
	setSelected: () => {}
})

export const CursorProvider = ({ children }: PropsWithChildren<{}>) => {
	const [isDragging, setIsDragging] = useState(false)
	const [hovered, setHovered] = useState<Square | null>(null)
	const [selected, setSelected] = useState<Piece | null>(null)
	return (
		<context.Provider
			value={{
				isDragging,
				setIsDragging,
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
