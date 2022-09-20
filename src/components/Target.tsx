import { Square } from "../types"

const Target = ({ square }: { square: Square }) => {
	return (
		<div
			style={{
				width: 100,
				height: 100,
				position: "absolute",
				top: 700 - square.rank * 100,
				left: square.file * 100
			}}>
			<div
				style={{
					width: "40px",
					height: "40px",
					margin: "30px",
					background: "rgba(0, 0, 0, 0.1)",
					borderRadius: "50%"
				}}
			/>
		</div>
	)
}

export default Target
