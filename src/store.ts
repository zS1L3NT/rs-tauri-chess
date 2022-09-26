import { configureStore } from "@reduxjs/toolkit"

import BoardSlice from "./slices/BoardSlice"
import CursorSlice from "./slices/CursorSlice"
import PromotionSlice from "./slices/PromotionSlice"

const store = configureStore({
	reducer: {
		cursor: CursorSlice,
		board: BoardSlice,
		promotion: PromotionSlice
	}
})

export default store
export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
