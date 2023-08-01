import { createSlice, PayloadAction } from "@reduxjs/toolkit"

import { Piece, Square } from "../types"

const slice = createSlice({
	name: "cursor",
	initialState: {
		isDragging: false,
		hovered: null as Square | null,
		selected: null as Piece | null,
	},
	reducers: {
		setCursor: (state, action: PayloadAction<Partial<typeof state>>) => {
			return { ...state, ...action.payload }
		},
	},
})

export default slice.reducer
export const { setCursor } = slice.actions
