import { createSlice, PayloadAction } from "@reduxjs/toolkit"

import { Board } from "../types"

const slice = createSlice({
	name: "board",
	initialState: {
		pieces: [],
		moves: [],
	} as Board,
	reducers: {
		setBoard: (state, action: PayloadAction<Board>) => {
			return action.payload
		},
	},
})

export default slice.reducer
export const { setBoard } = slice.actions
