import { Square } from "../types"

const Highlight = ({ square }: { square: Square }) => {
	return (
		<div
			style={{
				width: 100,
				height: 100,
				backgroundColor: "rgba(245, 204, 42, .5)",
				position: "absolute",
				top: 700 - square.rank * 100,
				left: square.file * 100
			}}
		/>
	)
}

export default Highlight
