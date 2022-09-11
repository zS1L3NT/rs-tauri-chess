import { Square } from "../types"

export default (a: Square | null, b: Square | null) => {
	if (a === null && b === null) return true
	if (a === null || b === null) return false
	return a.file === b.file && a.rank === b.rank
}
