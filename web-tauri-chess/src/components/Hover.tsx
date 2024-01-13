import useAppSelector from "../hooks/useAppSelector"

const Hover = () => {
	const hovered = useAppSelector(state => state.cursor.hovered)

	return hovered ? (
		<div
			style={{
				width: 100,
				height: 100,
				border: "5px solid rgba(255, 255, 255, 0.65)",
				position: "absolute",
				zIndex: 1,
				top: 700 - hovered.rank * 100,
				left: hovered.file * 100,
			}}
		/>
	) : (
		<></>
	)
}

export default Hover
