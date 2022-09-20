import { Square } from "../types"

const Target = ({ square, isCapture }: { square: Square; isCapture: boolean }) => {
	return (
		<div
			style={{
				width: 100,
				height: 100,
				position: "absolute",
				top: 700 - square.rank * 100,
				left: square.file * 100
			}}>
			{isCapture ? (
				<div
					style={{
						width: 100,
						height: 100,
						border: "10px solid rgba(0, 0, 0, 0.1)",
						borderRadius: "50%"
					}}
				/>
			) : (
				<div
					style={{
						width: "36px",
						height: "36px",
						margin: "32px",
						background: "rgba(0, 0, 0, 0.1)",
						borderRadius: "50%"
					}}
				/>
			)}
		</div>
	)
}

export default Target
