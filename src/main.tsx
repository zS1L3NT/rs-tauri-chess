import ReactDOM from "react-dom/client"

import App from "./App"
import { PiecesProvider } from "./contexts/PiecesContext"
import { CursorProvider } from "./contexts/CursorContext"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<PiecesProvider>
		<CursorProvider>
			<App />
		</CursorProvider>
	</PiecesProvider>
)
