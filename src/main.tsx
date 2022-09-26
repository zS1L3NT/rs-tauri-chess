import ReactDOM from "react-dom/client"
import { Provider as ReduxProvider } from "react-redux"

import App from "./App"
import store from "./store"

ReactDOM.createRoot(document.getElementById("root")!).render(
	<ReduxProvider store={store}>
		<App />
	</ReduxProvider>
)
