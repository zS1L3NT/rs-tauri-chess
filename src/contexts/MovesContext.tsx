import { createContext, PropsWithChildren, useState } from "react"

import { Move } from "../types"

type iMovesData = {
	moves: Move[]
	setMoves: (moves: Move[]) => void
}

const context = createContext<iMovesData>({
	moves: [],
	setMoves: () => {}
})

export const MovesProvider = ({ children }: PropsWithChildren<{}>) => {
	const [moves, setMoves] = useState<Move[]>([])

	return (
		<context.Provider
			value={{
				moves,
				setMoves
			}}>
			{children}
		</context.Provider>
	)
}

export default context
