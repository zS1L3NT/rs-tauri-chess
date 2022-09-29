const clean = str => {
	const perft = {}
	const lines = str
		.split("\n")
		.map(l => l.trim())
		.filter(l => !!l)

	for (const line of lines.sort()) {
		const [_, move, count] = line.match(/(\w{4}): (\d+)/)
		perft[move] = parseInt(count)
	}

	return perft
}

const stockfish = clean(`

`)

const engine = clean(`

`)

for (const move in stockfish) {
	if (move in engine) {
		if (stockfish[move] !== engine[move]) {
			console.log({
				move,
				stockfish: stockfish[move],
				engine: engine[move],
				offset: engine[move] - stockfish[move]
			})
		}
	} else {
		console.log({
			move,
			stockfish: stockfish[move],
			engine: null,
			offset: engine[move] - stockfish[move]
		})
	}
}

for (const move in engine) {
	if (!(move in stockfish)) {
		console.log({
			move,
			stockfish: null,
			engine: engine[move],
			offset: engine[move] - stockfish[move]
		})
	}
}
