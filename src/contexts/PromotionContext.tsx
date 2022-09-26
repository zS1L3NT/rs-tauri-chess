import { createContext, PropsWithChildren, useState } from "react"

import { Color, File, PieceType } from "../types"

type iPromotionData = {
	file: File | null
	setFile: (file: File | null) => void
	color: Color | null
	setColor: (color: Color | null) => void
	choice: PieceType | null
	setChoice: (choice: PieceType | null) => void
}

const context = createContext<iPromotionData>({
	file: null,
	setFile: () => {},
	color: null,
	setColor: () => {},
	choice: null,
	setChoice: () => {}
})

export const PromotionProvider = ({ children }: PropsWithChildren<{}>) => {
	const [file, setFile] = useState<File | null>(null)
	const [color, setColor] = useState<Color | null>(null)
	const [choice, setChoice] = useState<PieceType | null>(null)

	return (
		<context.Provider
			value={{
				file,
				setFile,
				color,
				setColor,
				choice,
				setChoice
			}}>
			{children}
		</context.Provider>
	)
}

export default context
