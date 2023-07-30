import { File, Rank, Square } from "../types"

export default (css: CSSStyleDeclaration): Square => {
	let transformX: string
	let transformY: string
	let left: string
	let top: string

	const transformMatch = css.transform.match(
		/translateX\((-?[\\.\d]+)px\) translateY\((-?[\\.\d]+)px\)/,
	)
	if (transformMatch) {
		transformX = transformMatch[1]
		transformY = transformMatch[2]
	} else {
		transformX = "0"
		transformY = "0"
	}

	const leftMatch = css.left.match(/([\\.\d]+)px/)
	if (leftMatch) {
		left = leftMatch[1]
	} else {
		throw new Error("Invalid CSS left")
	}

	const topMatch = css.top.match(/([\\.\d]+)px/)
	if (topMatch) {
		top = topMatch[1]
	} else {
		throw new Error("Invalid CSS top")
	}

	const draggedX = parseFloat(left) + parseFloat(transformX)
	const draggedY = parseFloat(top) + parseFloat(transformY)

	return {
		file: Math.round(draggedX / 100) as File,
		rank: (7 - Math.round(draggedY / 100)) as Rank,
	}
}
