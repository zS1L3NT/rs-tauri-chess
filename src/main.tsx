import ReactDOM from "react-dom/client"

import App from "./App"
import { CursorProvider } from "./contexts/CursorContext"
import { MovesProvider } from "./contexts/MovesContext"
import { PiecesProvider } from "./contexts/PiecesContext"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<PiecesProvider>
		<MovesProvider>
			<CursorProvider>
				<App />
			</CursorProvider>
		</MovesProvider>
	</PiecesProvider>
)
