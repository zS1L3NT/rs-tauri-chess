import { createContext, Dispatch, PropsWithChildren, SetStateAction, useState } from "react"

import { Color, File, Piece, Rank, Type } from "../types"

type iPiecesData = {
	pieces: Piece[]
	setPieces: Dispatch<SetStateAction<Piece[]>>
}

const context = createContext<iPiecesData>({
	pieces: [],
	setPieces: () => {}
})

export const PiecesProvider = ({ children }: PropsWithChildren<{}>) => {
	const [pieces, setPieces] = useState<Piece[]>([
		{
			id: 60,
			color: Color.White,
			type: Type.King,
			square: {
				file: File.E,
				rank: Rank._1
			}
		}
	])

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
