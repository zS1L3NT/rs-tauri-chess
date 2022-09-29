const stockfish = [
	"f4f3: 1",
	"c5c4: 1",
	"d6d5: 1",
	"c5b4: 1",
	"f4e3: 1",
	"h5d5: 1",
	"h5e5: 1",
	"h5f5: 1",
	"h5g5: 1",
	"h5h6: 1",
	"h5h7: 1",
	"h5h8: 1",
	"h4g3: 1",
	"h4h3: 1",
	"h4g4: 1",
	"h4g5: 1"
]

const engine = [
	"d6d5: 1",
	"c5b4: 1",
	"c5c4: 1",
	"h5h6: 1",
	"h5h7: 1",
	"h5h8: 1",
	"h5g5: 1",
	"h5f5: 1",
	"h5e5: 1",
	"h5d5: 1",
	"f4f3: 1",
	"h4h3: 1",
	"h4g3: 1",
	"h4g4: 1",
	"h4g5: 1"
]

let i = 0
while (true) {
	if (!stockfish[i] && !engine[i]) {
		break
	}

	if (stockfish[i] !== engine[i]) {
		console.log({ stockfish: stockfish[i], engine: engine[i] })
	}

	i++
}
