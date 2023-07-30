import { createSlice, PayloadAction } from "@reduxjs/toolkit"

import { Color, File, Rank, Square } from "../types"

const slice = createSlice({
	name: "promotion",
	initialState: null as { file: File; color: Color } | null,
	reducers: {
		setPromotion: (state, action: PayloadAction<Square | null>) => {
			if (action.payload) {
				return {
					file: action.payload.file,
					color: action.payload.rank === Rank._8 ? Color.White : Color.Black,
				}
			} else {
				return null
			}
		},
	},
})

export default slice.reducer
export const { setPromotion } = slice.actions
