import { createContext, Dispatch, PropsWithChildren, SetStateAction, useState } from "react"

import defaultBoard from "../defaultBoard"
import { Piece } from "../types"

type iPiecesData = {
	pieces: Piece[]
	setPieces: Dispatch<SetStateAction<Piece[]>>
}

const context = createContext<iPiecesData>({
	pieces: [],
	setPieces: () => {}
})

export const PiecesProvider = ({ children }: PropsWithChildren<{}>) => {
	const [pieces, setPieces] = useState<Piece[]>(defaultBoard)

	return (
		<context.Provider
			value={{
				pieces,
				setPieces
			}}>
			{children}
		</context.Provider>
	)
}

export default context
